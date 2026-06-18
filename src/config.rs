// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use build::*;
use project::*;

use auth::mysql::*;
use databases::mysql::*;
use schema::mysql::*;

/// Silence's project config.
#[derive(
    Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, MutGetters, Patch, Debug,
)]
#[patch(attribute(derive(Clone, PartialEq, Constructor, Builder, Serialize, Deserialize, Debug)))]
#[getset(get = "pub", get_mut = "pub")]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    listening_addr: Option<SocketAddr>,

    /// Whether to serve static files in the project's `./static` folder.
    serve_static_files: bool,

    /// Defines the internal parameters.
    internal_params: InternalParams,

    /// Skips the endpoint discovery from these tables.
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    skip_discovery_tables: CheapVec<CompactString>,

    /// Skips discovering endpoints with these ids.
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    skip_endpoints_ids: CheapVec<CompactString>,

    /// Defines MySQL connection config.
    database_conn: MySQLDBConnectionConfig,

    /// Defines the order of the bootstrap scripts that will be executed first.
    /// NOTE: all the files in the `./bootstrap` will be executed.
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    bootstrap_scripts: CheapVec<CompactString>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listening_addr: Some(SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)),
            serve_static_files: true,
            internal_params: Default::default(),
            skip_discovery_tables: CheapVec::new(),
            skip_endpoints_ids: CheapVec::new(),
            database_conn: MySQLDBConnectionConfig::new(
                SocketAddr::new("127.0.0.1".parse().unwrap(), 3306),
                "iissi_user".to_compact_string(),
                "iissi$user".to_compact_string(),
                "example_db".to_compact_string(),
            ),
            bootstrap_scripts: Default::default(),
        }
    }
}

/// Defines the parameters that will be used internally to manage authentication and other routines.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Patch, Debug)]
#[patch(attribute(derive(Clone, PartialEq, Constructor, Builder, Serialize, Deserialize, Debug)))]
#[getset(get = "pub")]
pub struct InternalParams {
    user_id: CompactString,
    users_target_table: CompactString,
    sessions_target_table: CompactString,
    roles_target_table: CompactString,
}

impl Default for InternalParams {
    fn default() -> Self {
        Self {
            user_id: "usuarioId".to_compact_string(),
            users_target_table: "users".to_compact_string(),
            sessions_target_table: "sessions".to_compact_string(),
            roles_target_table: "roles".to_compact_string(),
        }
    }
}

impl Config {
    pub fn into_database_config(&self) -> DatabaseConfig {
        let mut skip_tables = self.skip_discovery_tables().to_owned();
        skip_tables.append(&mut CheapVec::<CompactString>::from_vec(vec![
            self.internal_params.sessions_target_table.to_owned(),
            self.internal_params.roles_target_table.to_owned(),
        ]));

        DatabaseConfig::new(
            "main".to_compact_string(),
            true,
            Arc::new(self.database_conn.to_owned()),
            Some(DataSchemaDiscoveryConfig::new(
                Arc::new(MySQLSchemaDiscoveryMethod::new(skip_tables)),
                true,
                false,
            )),
            None,
            None,
        )
    }

    pub fn into_build(self, endpoints: Endpoints) -> ExecutorBuild {
        ExecutorBuild::new(
            project::Config::new(
                "Silence App".to_compact_string(),
                CheapVec::from_vec(vec![self.into_database_config()]),
                Some(Authentication::new(
                    CheapVec::from_vec(vec![Arc::new(MySQLSimpleAuthenticationMethod::new(
                        None,
                        self.internal_params.users_target_table.to_owned(),
                        self.internal_params.user_id.to_owned(),
                        "email".to_compact_string(),
                        "password".to_compact_string(),
                        None,
                    ))]),
                    Arc::new(MySQLToken::new(
                        None,
                        self.internal_params.sessions_target_table.to_owned(),
                        "token_id".to_compact_string(),
                        self.internal_params.user_id.to_owned(),
                        "created_at".to_compact_string(),
                        3600,
                    )),
                    Some(Arc::new(MySQLRole::new(
                        None,
                        self.internal_params.roles_target_table.to_owned(),
                        self.internal_params.user_id.to_owned(),
                        "role".to_compact_string(),
                    ))),
                    None,
                    true,
                    false,
                )),
                Admin::new(
                    false,
                    CheapVec::from_vec(vec!["admin".to_compact_string()]),
                    false,
                ),
            ),
            Executor::new(
                *self.listening_addr(),
                None,
                "/api".to_compact_string(),
                false,
                0,
            ),
            endpoints,
            CheapVec::new(),
        )
    }
}
