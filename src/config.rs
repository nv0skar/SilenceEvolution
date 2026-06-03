// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use build::*;
use project::*;

use auth::mysql::*;
use databases::mysql::*;
use schema::mysql::*;

/// Silence's project config.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    listening_addr: Option<SocketAddr>,

    /// Whether to serve static files in the project's `./static` folder.
    serve_static_files: bool,

    /// Defines auth config.
    origin_table: OriginTable,

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
            origin_table: Default::default(),
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

/// Defines the users and sessions table names.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct OriginTable {
    user_id: CompactString,
    users_table: CompactString,
    sessions: CompactString,
    roles: CompactString,
}

impl Default for OriginTable {
    fn default() -> Self {
        Self {
            user_id: "usuarioId".to_compact_string(),
            users_table: "users".to_compact_string(),
            sessions: "sessions".to_compact_string(),
            roles: "roles".to_compact_string(),
        }
    }
}

impl Config {
    pub fn into_database_config(&self) -> DatabaseConfig {
        DatabaseConfig::new(
            "main".to_compact_string(),
            true,
            Arc::new(self.database_conn.to_owned()),
            Some(DataSchemaDiscoveryConfig::new(
                Arc::new(MySQLSchemaDiscoveryMethod::new(CheapVec::from_vec(vec![
                    self.origin_table.sessions.to_owned(),
                    self.origin_table.roles.to_owned(),
                ]))),
                true,
                false,
            )),
            None,
            None,
        )
    }

    pub fn into_build(self, endpoints: wv_endpoint::Endpoints) -> ExecutorBuild {
        ExecutorBuild::new(
            project::Config::new(
                "Silence App".to_compact_string(),
                CheapVec::from_vec(vec![self.into_database_config()]),
                Some(Authentication::new(
                    CheapVec::from_vec(vec![Arc::new(MySQLSimpleAuthenticationMethod::new(
                        None,
                        self.origin_table.users_table.to_owned(),
                        self.origin_table.user_id.to_owned(),
                        "email".to_compact_string(),
                        "password".to_compact_string(),
                        None,
                    ))]),
                    Arc::new(MySQLToken::new(
                        None,
                        self.origin_table.sessions.to_owned(),
                        "token_id".to_compact_string(),
                        self.origin_table.user_id.to_owned(),
                        "created_at".to_compact_string(),
                        3600,
                    )),
                    Some(Arc::new(MySQLRole::new(
                        None,
                        self.origin_table.roles.to_owned(),
                        self.origin_table.user_id.to_owned(),
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
