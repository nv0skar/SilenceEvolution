// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use object::*;
use project::*;

use waveless_sql::auth::mysql::*;
use waveless_sql::databases::mysql::*;

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
    internal_params: Option<InternalParams>,

    /// Skips the endpoint discovery from these tables.
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    skip_discovery_tables: CheapVec<CompactString>,

    /// Skips discovering endpoints with these ids.
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    skip_endpoints_ids: CheapVec<CompactString>,

    /// Defines MySQL connection config.
    databases_conn: Option<DatabasesConnectionConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listening_addr: Some(SocketAddr::new("127.0.0.1".parse().unwrap(), 8080)),
            serve_static_files: true,
            internal_params: Default::default(),
            skip_discovery_tables: CheapVec::new(),
            skip_endpoints_ids: CheapVec::new(),
            databases_conn: Default::default(),
        }
    }
}

/// Defines the parameters that will be used internally to manage authentication and other routines.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Patch, Args, Debug)]
#[patch(attribute(derive(Clone, PartialEq, Constructor, Builder, Serialize, Deserialize, Debug)))]
#[getset(get = "pub")]
pub struct InternalParams {
    #[command(flatten)]
    users: UsersTable,

    #[arg(long = "roles_table", help = "Set the Silence's roles table.")]
    roles: CompactString,
}

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Args, Debug)]
#[getset(get = "pub")]
pub struct UsersTable {
    #[arg(long = "users_table", help = "Set the Silence's users table.")]
    table: CompactString,

    #[command(flatten)]
    fields: UsersTableFields,
}

#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Args, Debug)]
#[getset(get = "pub")]
pub struct UsersTableFields {
    #[arg(long = "users_id_field", help = "Set the `user_id` field.")]
    id: CompactString,

    #[arg(long = "users_name_field", help = "Set the `name` field.")]
    name: CompactString,

    #[arg(long = "users_email_field", help = "Set the `email` field.")]
    email: CompactString,

    #[arg(long = "users_password_field", help = "Set the `password` field.")]
    password: CompactString,
}

impl Default for UsersTableFields {
    fn default() -> Self {
        Self {
            id: "user_id".into(),
            name: "name".into(),
            email: "email".into(),
            password: "password".into(),
        }
    }
}

impl Default for UsersTable {
    fn default() -> Self {
        Self {
            table: "silence_users".into(),
            fields: Default::default(),
        }
    }
}

impl Default for InternalParams {
    fn default() -> Self {
        Self {
            users: Default::default(),
            roles: "silence_roles".into(),
        }
    }
}

/// TODO: add docs here.
#[derive(
    Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, MutGetters, Patch, Debug,
)]
#[patch(attribute(derive(Clone, PartialEq, Constructor, Builder, Serialize, Deserialize, Debug)))]
#[getset(get = "pub", get_mut = "pub")]
pub struct DatabasesConnectionConfig {
    main: MySQLDbConnectionConfig,
    internal: Option<MySQLDbConnectionConfig>,
}

impl Default for DatabasesConnectionConfig {
    fn default() -> Self {
        Self {
            main: MySQLDbConnectionConfig::new(
                SocketAddr::new("127.0.0.1".parse().unwrap(), 3306),
                "iissi_user".to_compact_string(),
                "iissi$user".to_compact_string(),
                "example_db".to_compact_string(),
            ),
            internal: Some(MySQLDbConnectionConfig::new(
                SocketAddr::new("127.0.0.1".parse().unwrap(), 3306),
                "iissi_user".to_compact_string(),
                "iissi$user".to_compact_string(),
                "silence_db".to_compact_string(),
            )),
        }
    }
}

impl Config {
    pub fn into_database_config(&self) -> CheapVec<DatabaseConfig> {
        let mut database_configs = CheapVec::new();

        let mut skip_tables = self.skip_discovery_tables().to_owned();

        if let Some(internal_params) = self.internal_params.to_owned() {
            skip_tables.append(&mut CheapVec::<CompactString>::from_vec(vec![
                internal_params.users.table.to_owned(),
                internal_params.roles.to_owned(),
            ]));
        }

        if let Some(databases_conn) = self.databases_conn.to_owned() {
            database_configs.push(DatabaseConfig::new(
                "main".to_compact_string(),
                true,
                Arc::new(databases_conn.main().to_owned()),
                None,
                None,
            ));

            database_configs.push(DatabaseConfig::new(
                "internal".to_compact_string(),
                false,
                Arc::new(match databases_conn.internal() {
                    Some(internal_db_config) => internal_db_config.to_owned(),
                    None => databases_conn.main().to_owned(),
                }),
                None,
                None,
            ));
        }

        database_configs
    }

    pub fn into_build(self, endpoints: Endpoints) -> ObjectArtifact {
        ObjectArtifact::new(
            project::Config::new(
                "Silence App".into(),
                self.into_database_config(),
                match self.internal_params() {
                    Some(internal_params) => Some(Authentication::new(
                        CheapVec::from_vec(vec![Arc::new(MySQLSimpleAuthentication::new(
                            Some("internal".into()),
                            internal_params.users.table.to_owned(),
                            internal_params.users.fields.id.to_owned(),
                            internal_params.users.fields.email.to_owned(),
                            internal_params.users.fields.password.to_owned(),
                            PasswordHashBackend::Plain,
                            CheapVec::from_vec(vec![internal_params.users.fields.name.to_owned()]),
                            None,
                        ))]),
                        Arc::new(backend::session_manager::InMemoryToken),
                        Some(Arc::new(MySQLRole::new(
                            Some("internal".into()),
                            internal_params.roles.to_owned(),
                            "user_id".into(),
                            "role".into(),
                        ))),
                        None,
                        true,
                        true,
                    )),
                    None => None,
                },
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
