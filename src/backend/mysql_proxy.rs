// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use config::*;

use databases::*;

use http_executor::{request_cx::*, *};

use waveless_sql::http_executor::{mysql::*, *};

/// Proxies MySQL execute queries and injects internal params on runtime.
#[derive(
    Clone, PartialEq, Constructor, Serialize, Deserialize, BoxedAny, Getters, Display, Debug,
)]
#[display("SQL Proxy: {:?}", _0)]
#[getset(get = "pub")]
#[serde(transparent)]
pub struct MySQLExecutorProxy(MySQLExecutor);

impl Default for MySQLExecutorProxy {
    fn default() -> Self {
        Self(SQLQueryWrapper::new("SELECT * FROM example".into()).into())
    }
}

impl From<MySQLExecutor> for MySQLExecutorProxy {
    fn from(execute: MySQLExecutor) -> Self {
        Self(execute)
    }
}

impl AnyExt for MySQLExecutorProxy {
    fn name(&self) -> &str {
        "silence_mysql_proxy"
    }
}

#[typetag::serde(name = "MySQLProxy")]
#[async_trait]
impl AnyHttpExecutor for MySQLExecutorProxy {
    async fn execute(&self, cx: PipelineCx, db_conns: DbConns) -> PipelineResult {
        let MySQLExecutorProxy(mysql_execute) = self;

        let PipelineCx { mut request, .. } = cx;

        let RequestCx { request_params, .. } = &mut request;

        // Inject runtime parameters.
        let internal_params = AppCx::acquire()
            .config()
            .read()
            .await
            .internal_params()
            .to_owned()
            .unwrap_or(InternalParams::default());

        let injections = [
            ("users_table", internal_params.users().table()),
            ("user_id_field", internal_params.users().fields().id()),
            ("user_name_field", internal_params.users().fields().name()),
            ("user_email_field", internal_params.users().fields().email()),
            (
                "user_password_field",
                internal_params.users().fields().password(),
            ),
            ("roles_table", internal_params.roles()),
        ];

        for (key, val) in injections {
            request_params.insert(key.into(), ParamValue::Internal(val.to_owned()));
        }

        mysql_execute
            .execute(
                PipelineCx {
                    request,
                    response: None,
                },
                db_conns,
            )
            .await
    }
}
