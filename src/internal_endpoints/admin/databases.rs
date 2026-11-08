// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use http_executor::{request_cx::*, *};
use project::*;

use waveless_sql::databases::mysql::*;

#[derive(Clone, Serialize, Deserialize, BoxedAny, Getters, Display, Debug)]
#[display("Databases manager.")]
pub struct DatabasesManager(pub(crate) DatabasesManagerOperation);

#[derive(Clone, Serialize, Deserialize, BoxedAny, Display, Debug)]
pub enum DatabasesManagerOperation {
    List,
    Get,
    Load,
    Remove,
    Restore,
}

impl AnyExt for DatabasesManager {
    fn name(&self) -> &str {
        "silence_databases"
    }
}

/// TODO: add docs here.
#[typetag::serde(name = "DatabasesManager")]
#[async_trait]
impl AnyHttpExecutor for DatabasesManager {
    async fn execute(&self, cx: PipelineCx, _db_conns: DbConns) -> PipelineResult {
        let PipelineCx {
            mut request,
            response,
        } = cx;

        let RequestCx { request_params, .. } = &mut request;

        let mut response = response.unwrap_or_default();

        *response.body_mut() = None; // Empty the response body set by previous execution steps.

        let DatabasesManager(operation) = self;

        match operation {
            DatabasesManagerOperation::List => {
                let db_manager = databases::DatabasesManager::acquire();

                let _db_conns: DbConns = db_manager.into();

                let db_conns = _db_conns
                    .iter()
                    .map(|(key, value)| {
                        let db_conn = value
                            .to_owned()
                            .into_arc_any()
                            .downcast::<MySQLConnection>()
                            .unwrap();

                        let mysql_conn_options =
                            db_conn.get().get_mysql_connection_pool().connect_options();

                        (
                            key,
                            json!({
                                "host": &mysql_conn_options.get_host(),
                                "port": mysql_conn_options.get_port(),
                                "username": &mysql_conn_options.get_username(),
                                "db": &mysql_conn_options.get_database(),
                            }),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let serialized_db = serde_json::to_value(db_conns).map_err(|err| {
                    RequestError::Other(
                        eyre!(err).wrap_err("Cannot serialize databases connections."),
                    )
                })?;

                *response.body_mut() = Some(BodyValue::Json(serialized_db));

                Ok((
                    PipelineCx {
                        request,
                        response: Some(response),
                    },
                    PipelineAction::Continue(None),
                ))
            }
            DatabasesManagerOperation::Get => {
                let db_manager = databases::DatabasesManager::acquire();

                let Some(ParamValue::Client(Some(id))) = request_params.get("id") else {
                    Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        "Missing database id param.".into(),
                    ))?
                };

                let db_conn = db_manager
                    .search(id)?
                    .to_owned()
                    .into_arc_any()
                    .downcast::<MySQLConnection>()
                    .unwrap();

                let mysql_conn_options =
                    db_conn.get().get_mysql_connection_pool().connect_options();

                *response.body_mut() = Some(BodyValue::Json(json!({
                    "host": &mysql_conn_options.get_host(),
                    "port": mysql_conn_options.get_port(),
                    "username": &mysql_conn_options.get_username(),
                    "db": &mysql_conn_options.get_database(),
                })));

                Ok((
                    PipelineCx {
                        request,
                        response: Some(response),
                    },
                    PipelineAction::Continue(None),
                ))
            }
            DatabasesManagerOperation::Load => {
                let db_manager = databases::DatabasesManager::acquire();

                let (
                    Some(ParamValue::Client(Some(id))),
                    Some(ParamValue::Client(Some(host))),
                    Some(ParamValue::Client(Some(username))),
                    Some(ParamValue::Client(Some(password))),
                    Some(ParamValue::Client(Some(db))),
                ) = (
                    request_params.get("id"),
                    request_params.get("host"),
                    request_params.get("username"),
                    request_params.get("password"),
                    request_params.get("db"),
                )
                else {
                    Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        "Missing request params.".into(),
                    ))?
                };

                let db_conn_config = DatabaseConfig::new(
                    id.to_owned(),
                    false,
                    Arc::new(MySQLDbConnectionConfig::new(
                        host.to_string().parse::<SocketAddr>().map_err(|_| {
                            RequestError::Expected(
                                StatusCode::BAD_REQUEST,
                                "Cannot parse the host.".into(),
                            )
                        })?,
                        username.to_owned(),
                        password.to_owned(),
                        db.to_owned(),
                    )),
                    None,
                    None,
                );

                db_manager.load(db_conn_config).await?;

                Ok((
                    PipelineCx {
                        request,
                        response: Some(response),
                    },
                    PipelineAction::Continue(None),
                ))
            }
            DatabasesManagerOperation::Remove => {
                let db_manager = databases::DatabasesManager::acquire();

                let Some(ParamValue::Client(Some(id))) = request_params.get("id") else {
                    Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        "Missing database id param.".into(),
                    ))?
                };

                db_manager.remove(id).await?;

                Ok((
                    PipelineCx {
                        request,
                        response: Some(response),
                    },
                    PipelineAction::Continue(None),
                ))
            }
            DatabasesManagerOperation::Restore => {
                let app_cx = AppCx::acquire();

                let db_manager = databases::DatabasesManager::acquire();

                db_manager
                    .load_all(app_cx.config().read().await.into_database_config())
                    .await?;

                Ok((
                    PipelineCx {
                        request,
                        response: Some(response),
                    },
                    PipelineAction::Continue(None),
                ))
            }
        }
    }
}
