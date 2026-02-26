// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

//!
//! The Silence's app context.
//! In order to manage Silence's app hot-reloading, a global application context is needed in order
//! to sync the Silence project's files and Waveless' runtime context.
//! The main use of this API is for Silence's internal executors. As an example, let's visualize
//! the inner workings of `NewEndpoint`, the internal endpoint that adds new user's endpoints:
//! `NewEndpoint` is defined at path `/manage/endpoints` with the POST method, and set to capture
//! all request's params, enabling the handler to manage any arbitrary params, thus the endpoint
//! does not require to be tied with the actual endpoint schema definition (dynamic parameters).
//! The workflow is the following:
//! 1. Deserializes the JSON serialized request into `silence::endpoint::Endpoint`.
//! 2. Converts the `silence::endpoint::Endpoint` definition back to `waveless_commons::endpoint::Endpoint`.
//! 3. Acquires `waveless_executor::RuntimeCx`, adds the new Waveless' endpoint and rebuilds the router manager.
//! 4. If the previous step succeeded, appends the `silence::endpoint::Endpoint` definition of the endpoint
//! to `silence::AppCx` ― if it failed, there is an equivalent endpoint already loaded, so the new endpoint
//! is not valid.
//! 5. Serializes `CheapVec<silence::endpoint::Endpoint>` back into JSON and saves it into the file.
//!

use crate::*;

use endpoint::{internal::INTERNAL_ENDPOINTS, *};

use waveless_commons::{databases::*, get_workspace_root};
use waveless_compiler::{discovery::*, *};
use waveless_executor::*;

#[derive(Constructor, Getters, Debug)]
#[getset(get = "pub")]
pub struct AppCx {
    config: RwLock<config::Config>,
    user_endpoints: RwLock<CheapVec<(CompactString, CheapVec<Endpoint>)>>,
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
    pub async fn set_cx(self) -> Result<()> {
        RuntimeCx::from_build(
            self.config()
                .read()
                .await
                .to_owned()
                .into_build(self.build_endpoints().await?),
        )
        .await?
        .set_cx();

        if !APP_CX.initialized() {
            APP_CX.set(self).unwrap();
        } else {
            panic!("App's context has already been initialized.");
        }

        Ok(())
    }

    /// Loads the Silence's app's database into the databases manager (`waveless_commons::databases::DatabasesConnections`).
    pub async fn set_databases<'app_guard>(
        app_cx: RwLockReadGuard<'app_guard, AppCx>,
    ) -> Result<()> {
        DatabasesConnections::load(CheapVec::from_vec(vec![
            app_cx.config.read().await.into_database_config(),
        ]))
        .await
    }

    /// Loads the Silence's app's settings into the Waveless' runtime context.
    /// NOTE: this method requires `waveless_executor::RuntimeCx` to be initialized otherwise it will panic.
    pub async fn set_config<'app_guard, 'runtime_guard>(
        app_cx: RwLockReadGuard<'app_guard, AppCx>,
        current_cx: RwLockWriteGuard<'runtime_guard, RuntimeCx>,
    ) -> Result<()> {
        let virtual_build = app_cx
            .config
            .read()
            .await
            .to_owned()
            .into_build(waveless_commons::endpoint::Endpoints::new(CheapVec::new()));

        let mut current_build = current_cx.build().write().await;

        *current_build.config_mut() = virtual_build.config().to_owned();
        *current_build.executor_mut() = virtual_build.executor().to_owned();
        *current_build.databases_checksums_mut() = virtual_build.databases_checksums().to_owned();

        Ok(())
    }

    /// Loads the Silence's app's endpoints into the Waveless' runtime context.
    /// NOTE: this method requires `waveless_executor::RuntimeCx` to be initialized otherwise it will panic.
    pub async fn set_endpoints() -> Result<()> {
        let app_cx = Self::acquire();

        let current_cx = RuntimeCx::acquire();

        let endpoints = app_cx.build_endpoints().await?;

        let mut current_build = current_cx.build().write().await;

        *current_build.endpoints_mut() = endpoints;

        // Clean all previous routers.
        while let Some(entry) = current_cx.router().iter().next() {
            current_cx.router().remove(entry.key());
        }

        // Rebuild endpoints.
        current_cx.build_router().await?;

        Ok(())
    }

    /// Converts the Silence's app's endpoints into Waveless' endpoints.
    async fn build_endpoints(&self) -> Result<waveless_commons::endpoint::Endpoints> {
        // In the future, endpoint discovery will be `CompilerCx` independent.
        // In the meanwhile we do this as a workaround.
        let dummy_build = self
            .config
            .read()
            .await
            .to_owned()
            .into_build(waveless_commons::endpoint::Endpoints::new(CheapVec::new()));

        // Initialie CompilerCx if needed.
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

        // Loads user-defined endpoints.
        let user_endpoints = self.user_endpoints.read().await;

        let mut endpoints = waveless_commons::endpoint::Endpoints::new(
            user_endpoints
                .iter()
                .map(|(_, endpoints)| endpoints.to_owned())
                .flatten()
                .map(|endpoint| -> waveless_commons::endpoint::Endpoint { endpoint.into() })
                .collect::<CheapVec<waveless_commons::endpoint::Endpoint>>(),
        );

        // Adds internal endpoints.
        endpoints.merge(INTERNAL_ENDPOINTS.to_owned())?;

        // Discover MySQL schema endpoints.
        endpoints.merge(
            discover()
                .await?
                .0
                .iter()
                .filter(|(name, _)| name == "main".to_compact_string())
                .map(|(_, endpoints)| endpoints.to_owned())
                .next()
                .unwrap(),
        )?;

        Ok(endpoints)
    }

    /// Builds the app's context by loading the project
    /// from the workspace's root.
    pub async fn from_workspace() -> Result<Option<Self>> {
        let workspace_root = get_workspace_root("config.json").unwrap_or(current_dir()?);

        // Loads project's config.
        let config = match read(workspace_root.join("config.json")) {
            Ok(file_buffer) => match serde_json::from_slice::<config::Config>(&file_buffer) {
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

        // Loads project's endpoints (Borrowed from the `waveless_compiler`).
        let mut endpoints = CheapVec::<(CompactString, CheapVec<Endpoint>)>::new();

        {
            let endpoints_dir = workspace_root.join("endpoints");

            if let Ok(endpoints_path) = read_dir(endpoints_dir) {
                for endpoint_path in endpoints_path {
                    let endpoint_path = endpoint_path?;
                    match read(endpoint_path.path()) {
                        Ok(file_buffer) => {
                            match serde_json::from_slice::<CheapVec<Endpoint>>(&file_buffer) {
                                Ok(new_endpoints) => endpoints.push((
                                    endpoint_path.path().to_str().unwrap().to_compact_string(),
                                    new_endpoints,
                                )),
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
            } else {
                warn!(
                    "Endpoints directory (`./endpoints`) cannot be found. Omitting user endpoints..."
                )
            }
        }

        Ok(Some(Self {
            config: RwLock::new(config),
            user_endpoints: RwLock::new(endpoints),
            workspace_root,
        }))
    }
}
