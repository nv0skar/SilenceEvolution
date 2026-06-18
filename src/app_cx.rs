// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

//!
//! The Silence's app context.
//! In order to manage Silence's app hot-reloading, a global application context is needed in order
//! to sync the Silence project's files and Waveless' runtime context.
//! The main use of this API is for Silence's internal executors. As an example, let's visualize
//! the inner workings of `NewEndpoint`, the internal endpoint that adds new user's endpoints:
//! `NewEndpoint` is defined at path `/api/manage/admin/endpoint` with the POST method, and set to capture
//! all request's params, enabling the handler to manage any arbitrary params, thus the endpoint
//! does not require to be tied with the actual endpoint schema definition (dynamic parameters).
//! The workflow is the following:
//! 1. Deserializes the JSON serialized request into `silence::endpoint::UserEndpoint`.
//! 2. Converts the `silence::endpoint::Endpoint` definition back to `waveless_commons::endpoint::Endpoint`.
//! 3. Acquires `waveless_executor::RuntimeCx`, adds the new Waveless' endpoint and rebuilds the router manager.
//! 4. If the previous step succeeded, appends the `silence::endpoint::Endpoint` definition of the endpoint
//! to `silence::AppCx` ― if it failed, there is an equivalent endpoint already loaded, so the new endpoint
//! is not valid.
//! 5. Serializes `CheapVec<silence::endpoint::Endpoint>` back into JSON and saves it into the file.
//!

use crate::*;

use config::*;

use waveless_compiler::{COMPILER_CX, CompilerCx, discovery::*};

use databases::*;

pub type SimpleEndpointsByFile = CheapVec<(Option<PathBuf>, CheapVec<SimpleEndpoint>)>;

#[derive(Constructor, Getters, Debug)]
#[getset(get = "pub")]
pub struct AppCx {
    config: RwLock<Config>,
    #[getset(skip)]
    simple_endpoints_by_file: RwLock<SimpleEndpointsByFile>,
    workspace_root: PathBuf,
}

impl AppCx {
    pub fn acquire() -> &'static Self {
        APP_CX
            .get()
            .ok_or(anyhow!("App's context should have been initialized."))
            .unwrap()
    }

    /// Sets the `APP_CX`'s `OnceLock`. And sets Waveless' runtime context.
    /// NOTE: If app's context is set this method will panic.
    pub async fn set_global_cx(self) -> Result<()> {
        RuntimeCx::from_build(
            self.config()
                .read()
                .await
                .to_owned()
                .into_build(self.build_runtime_endpoints().await?),
        )
        .await?
        .set_cx();

        if !APP_CX.initialized() {
            APP_CX.set(self).unwrap();
        } else {
            panic!("App's context has already been initialized.");
        }

        // Loads the Silence's app's database into the databases manager (`waveless_commons::databases::DatabasesConnections`).
        DatabasesConnections::load(CheapVec::from_vec(vec![
            Self::acquire().config().read().await.into_database_config(),
        ]))
        .await?;

        Ok(())
    }

    /// Loads the Silence's app's settings into the Waveless' runtime context.
    /// NOTE: this method requires `waveless_executor::RuntimeCx` to be initialized otherwise it will panic.
    pub async fn set_runtime_config(&self) -> Result<()> {
        let mut runtime_build_guard = RuntimeCx::acquire().build().write().await;

        let virtual_build = self
            .config()
            .read()
            .await
            .to_owned()
            .into_build(waveless_commons::endpoint::Endpoints::new(CheapVec::new()));

        *runtime_build_guard.config_mut() = virtual_build.config().to_owned();
        *runtime_build_guard.executor_mut() = virtual_build.executor().to_owned();
        *runtime_build_guard.databases_checksums_mut() =
            virtual_build.databases_checksums().to_owned();

        Ok(())
    }

    /// Builds the app's context by loading the project
    /// from the workspace's root.
    pub async fn from_workspace() -> Result<Option<Self>> {
        let workspace_root = get_workspace_root("config.json").unwrap_or(current_dir()?);

        // Loads project's config.
        let config = match read(workspace_root.join("config.json")).await {
            Ok(file_buffer) => match serde_json::from_slice::<Config>(&file_buffer) {
                Ok(config) => config,
                Err(err) => bail!(
                    "Cannot deserialize the `config.json` file.%{}",
                    err.to_string()
                ),
            },
            Err(_) => {
                warn!("Cannot find the `config.json` file. A setup will be prompted.");
                return Ok(None);
            }
        };

        // Loads user-defined endpoints (Borrowed from the `waveless_compiler`).
        let endpoints = Self::get_simple_endpoints_files()
            .await?
            .unwrap_or_default();

        Ok(Some(Self {
            config: RwLock::new(config),
            simple_endpoints_by_file: RwLock::new(endpoints),
            workspace_root,
        }))
    }

    /// Syncs the app's config with the runtime build's config and saves current config state to the project's `config.json`.
    pub async fn set_config(&self) -> Result<()> {
        AppCx::acquire().set_runtime_config().await?;

        let config_guard = self.config().read().await;

        let workspace_root = get_workspace_root("config.json").unwrap_or(current_dir()?);
        let project_file = workspace_root.join("config.json");

        let content = serde_json::to_string_pretty(&config_guard.to_owned())?;

        write(project_file, content).await?;

        Ok(())
    }

    /// Converts the Silence's app's endpoints (`SimpleEndpoint`) into Waveless' endpoints and discovers the database's schema to automatically generate endpoints.
    async fn build_runtime_endpoints(&self) -> Result<waveless_commons::endpoint::Endpoints> {
        // In the future, endpoint discovery will be `CompilerCx` independent.
        // In the meanwhile we do this as a workaround.
        let dummy_build = self
            .config
            .read()
            .await
            .to_owned()
            .into_build(waveless_commons::endpoint::Endpoints::new(CheapVec::new()));

        // Initialize CompilerCx if needed.
        if !COMPILER_CX.initialized() {
            CompilerCx::new(
                waveless_commons::project::Project::new(
                    dummy_build.config().to_owned(),
                    waveless_commons::project::Compiler::new("".to_compact_string(), None, None),
                    dummy_build.executor().to_owned(),
                ),
                PathBuf::new(),
            )
            .set_cx();
        }

        // Converts user-defined endpoints.
        let simple_endpoints = self.simple_endpoints_by_file.read().await;

        let mut endpoints = waveless_commons::endpoint::Endpoints::new(
            simple_endpoints
                .iter()
                .map(|(_, endpoints)| endpoints.to_owned())
                .flatten()
                .map(|endpoint| -> waveless_commons::endpoint::Endpoint { endpoint.into() })
                .collect::<CheapVec<waveless_commons::endpoint::Endpoint>>(),
        );

        // Adds internal endpoints.
        endpoints.merge(INTERNAL_ENDPOINTS.to_owned())?;

        // Discover MySQL schema endpoints.
        let config_guard = self.config.read().await;

        let skip_endpoint_ids = config_guard.skip_endpoints_ids();

        endpoints.merge(
            discover()
                .await?
                .0
                .iter()
                .filter(|(name, _)| name == "main".to_compact_string())
                .cloned()
                .map(|(_, endpoints)| {
                    Endpoints::new(
                        endpoints
                            .inner()
                            .iter()
                            .cloned()
                            .filter(|endpoint| !skip_endpoint_ids.contains(endpoint.id()))
                            .collect::<CheapVec<Endpoint>>(),
                    )
                }) // won't add endpoints whose id is in `Config::skip_endpoints_ids`.
                .next()
                .unwrap(),
        )?;

        Ok(endpoints)
    }

    /// Loads simple endpoints files (Borrowed from the `waveless_compiler`).
    async fn get_simple_endpoints_files() -> Result<Option<SimpleEndpointsByFile>> {
        let mut endpoints = SimpleEndpointsByFile::new();

        let workspace_root = get_workspace_root("config.json").unwrap_or(current_dir()?);
        let endpoints_dir = workspace_root.join("endpoints");

        if let Ok(mut endpoints_path) = read_dir(endpoints_dir).await {
            while let Ok(Some(endpoint_path)) = endpoints_path.next_entry().await {
                match read(endpoint_path.path()).await {
                    Ok(file_buffer) => {
                        match serde_json::from_slice::<CheapVec<SimpleEndpoint>>(&file_buffer) {
                            Ok(new_endpoints) => {
                                // Check whether an endpoint has been auto-generated.
                                let mut _new_endpoints_iter = new_endpoints.iter();
                                while let Some(endpoint) = _new_endpoints_iter.next() {
                                    if *endpoint.auto_generated() {
                                        bail!(
                                            "The endpoint `{}` from file `{}` cannot be marked as auto-generated.",
                                            endpoint.id(),
                                            endpoint_path.path().display()
                                        );
                                    }
                                }
                                endpoints.push((Some(endpoint_path.path()), new_endpoints))
                            }
                            Err(err) => {
                                Err(anyhow!(
                                    "Cannot deserialize the endpoints definition file '{}'.%{}",
                                    endpoint_path.file_name().display(),
                                    err.to_string()
                                ))?;
                            }
                        };
                    }
                    Err(err) => {
                        Err(anyhow!(
                            "Cannot open the endpoints definition file '{}'.%{}",
                            endpoint_path.file_name().display(),
                            err.to_string()
                        ))?;
                    }
                }
            }

            debug!("Deserialized user's endpoints: {:#?}", endpoints);

            Ok(Some(endpoints))
        } else {
            warn!(
                "Endpoints directory (`./endpoints`) cannot be found. Omitting user endpoints..."
            );

            Ok(None)
        }
    }

    /// Sets simple endpoints files from the app's context.
    async fn set_simple_endpoints_files(&self, target_path: Option<PathBuf>) -> Result<()> {
        let workspace_root = get_workspace_root("config.json").unwrap_or(current_dir()?);

        // Create the `endpoint` directory if necessary.
        let _ = create_dir(workspace_root.join("endpoints")).await; // if an error occurs it will be ignored.

        match target_path {
            Some(target_path) => {
                // Update only a specific file.
                let simple_endpoints_by_file_guard = self.simple_endpoints_by_file.read().await;
                match simple_endpoints_by_file_guard.iter().find(|(opt_file, _)| {
                    opt_file
                        .to_owned()
                        .map(|file| file == *target_path)
                        .unwrap_or(false)
                }) {
                    Some((_, simple_endpoints)) => {
                        let content = serde_json::to_string_pretty(&simple_endpoints)?;

                        write(target_path, content).await?;

                        Ok(())
                    }
                    None => Err(anyhow!(
                        "Target path does not exist in the current app context."
                    )),
                }
            }
            None => {
                // Update all endpoint files.
                let simple_endpoints_by_file_guard = self.simple_endpoints_by_file.read().await;

                for (target_path, simple_endpoints) in simple_endpoints_by_file_guard.iter() {
                    if let Some(target_path) = target_path {
                        let content = serde_json::to_string_pretty(&simple_endpoints)?;

                        write(target_path, content).await?;
                    }
                }
                Ok(())
            }
        }
    }

    /// Returns the user-defined (`SimpleEndpoint`), the auto-generated and internal endpoints that have been loaded into the app.
    /// NOTE: the internal login endpoint will be injected in this method.
    pub async fn get_endpoints(&self) -> Result<SimpleEndpointsByFile> {
        let executor_build_guard = RuntimeCx::acquire().build().read().await;

        // Get user-defined endpoints by file.
        let mut simple_endpoints_by_file = self
            .simple_endpoints_by_file
            .read()
            .await
            .to_owned()
            .iter()
            .cloned()
            .collect::<SimpleEndpointsByFile>();

        // Get all the endpoints that have been injected and automatically generated.
        let endpoints = executor_build_guard
            .endpoints()
            .inner()
            .iter()
            .filter(|endpoint| *endpoint.auto_generated())
            .map(|endpoint| endpoint.into())
            .collect::<CheapVec<SimpleEndpoint>>();

        // Merge all the endpoints.
        simple_endpoints_by_file.push((None, endpoints));

        Ok(simple_endpoints_by_file)
    }

    /// Add an endpoint that will be loaded into the runtime and saved into the given file.
    /// NOTE: any modification of the endpoint file that hasn't been loaded into the app context will be lost.
    pub async fn add_endpoint(
        &self,
        target: CompactString,
        endpoint: SimpleEndpoint,
    ) -> Result<()> {
        if !target.ends_with(".json") {
            bail!("Target file doesn't end with `.json`.");
        }

        if *endpoint.auto_generated() {
            bail!("The endpoint cannot be marked as auto-generated.");
        }

        // Add the endpoint to the runtime context.
        {
            let mut executor_build_guard = RuntimeCx::acquire().build().write().await;
            let endpoints = executor_build_guard.endpoints_mut();

            endpoints.add(endpoint.to_owned().into())?; // This can fail, as the endpoint might already exist.
        }

        // Rebuild the router.
        RuntimeCx::acquire().build_router().await?;

        // Add the endpoint to the Silence's app context.
        let workspace_root = get_workspace_root("config.json").unwrap_or(current_dir()?);

        let target_path = workspace_root.join("endpoints").join(&target);

        {
            let mut simple_endpoint_by_file_guard = self.simple_endpoints_by_file.write().await;

            match simple_endpoint_by_file_guard
                .iter_mut()
                .find(|(opt_file, _)| {
                    opt_file
                        .to_owned()
                        .map(|file| file == target_path)
                        .unwrap_or(false)
                }) {
                Some((_, simple_endpoints)) => {
                    // This flow assumes that the file already exists.
                    simple_endpoints.push(endpoint);
                }
                None => {
                    // While in this flow is not guaranteed that the file nor the directory exists.
                    let endpoints = CheapVec::from_vec(vec![endpoint.to_owned()]);

                    simple_endpoint_by_file_guard.push((Some(target_path.to_owned()), endpoints));
                }
            }
        }

        self.set_simple_endpoints_files(Some(target_path)).await?;

        Ok(())
    }

    /// Modify endpoint's attributes.
    pub async fn set_endpoint(&self, id: CompactString, patch: SimpleEndpointPatch) -> Result<()> {
        let endpoints = RuntimeCx::acquire()
            .build()
            .write()
            .await
            .endpoints()
            .to_owned();

        let Some(endpoint) = endpoints
            .inner()
            .iter()
            .cloned()
            .find(|endpoint| *endpoint.id() == id)
        else {
            bail!("Couldn't find a loaded endpoint with the id {}", id);
        };

        // Check whether we are trying to modify an internal endpoint.
        if is_endpoint_internal(&endpoint) {
            bail!("Cannot modify an internal endpoint `{}`.", id);
        }

        // Convert endpoint from `waveless_commons::endpoint::Endpoint` back to Silence's `SimpleEndpoint`.
        let mut simple_endpoint = SimpleEndpoint::from(endpoint);
        *simple_endpoint.auto_generated_mut() = false;

        // Delete the endpoint.
        let target_path = self.delete_endpoint(id).await?;

        // Add the new endpoint.
        simple_endpoint.apply(patch);

        self.add_endpoint(
            target_path
                .map(|target_path| {
                    target_path
                        .iter()
                        .last()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_compact_string()
                })
                .unwrap_or("default.json".to_compact_string()),
            simple_endpoint,
        )
        .await?;

        Ok(())
    }

    /// Delete an endpoint from the runtime and from its file given its id.
    pub async fn delete_endpoint(&self, id: CompactString) -> Result<Option<PathBuf>> {
        let mut build = RuntimeCx::acquire().build().write().await;

        let endpoints = build.endpoints_mut().inner_mut();

        let Some((ix, endpoint)) = endpoints
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, endpoint)| *endpoint.id() == id)
        else {
            bail!("Couldn't find a loaded endpoint with the id {}", id);
        };

        // Check whether an internal endpoint is being removed.
        if is_endpoint_internal(&endpoint) {
            bail!("Cannot remove internal endpoint `{}`.", id);
        };

        // Remove the endpoint from the runtime context.
        let _ = endpoints.remove(ix);

        let mut target_path = None;

        // If the endpoint is auto-generated, the specific endpoint id will be skipped in subsequent endpoint discoveries.
        // Beware that by definition, the whole table isn't skipped only that specific endpoint.
        // Otherwise, if the endpoint is user-generated remove the endpoint from it's definition file.
        if *endpoint.auto_generated() {
            let mut _config_guard = Self::acquire().config().write().await;
            _config_guard
                .skip_endpoints_ids_mut()
                .push(endpoint.id().to_owned());
        } else {
            // Find the simple endpoint and remove it.
            let mut simple_endpoint_by_files =
                Self::acquire().simple_endpoints_by_file.write().await;

            for (target_path_iter, simple_endpoints) in simple_endpoint_by_files.iter_mut() {
                if let Some(target_path_iter) = target_path_iter {
                    let Some((ix, _)) = simple_endpoints
                        .iter()
                        .enumerate()
                        .find(|(_, endpoint)| *endpoint.id() == id)
                    else {
                        continue;
                    };

                    simple_endpoints.remove(ix);

                    target_path = Some(target_path_iter.to_owned());

                    break;
                }
            }

            if let Some(target_path) = target_path.to_owned() {
                drop(simple_endpoint_by_files); // drop the lock to be able to read the simple endpoints from the app's context.
                self.set_simple_endpoints_files(Some(target_path)).await?;
            }
        }

        drop(build); // drop the lock to be able to write to the runtime build.

        RuntimeCx::acquire().build_router().await?; // Rebuild the runtime's router.

        // If the endpoint was auto-generated it means that the app's config has been modified,
        // we have to set the config (cannot be done before because the lock is held by `build`).
        if *endpoint.auto_generated() {
            AppCx::acquire().set_config().await?;
        }

        Ok(target_path)
    }
}
