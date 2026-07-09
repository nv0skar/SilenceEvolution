// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use http_execute::{mysql::*, request_cx::*, *};

/// Proxies MySQL execute queries and injects internal params on runtime.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Display, Debug)]
#[display("SQL Proxy: {:?}", _0)]
#[getset(get = "pub")]
#[serde(transparent)]
pub struct MySQLExecuteProxy(MySQLExecute);

boxed_any!(MySQLExecuteProxy);

impl Default for MySQLExecuteProxy {
    fn default() -> Self {
        Self(MySQLQueryWrapper::new("SELECT * FROM example".into()).into())
    }
}

impl From<MySQLExecute> for MySQLExecuteProxy {
    fn from(execute: MySQLExecute) -> Self {
        Self(execute)
    }
}

#[typetag::serde(name = "MySQLProxy")]
#[async_trait]
impl AnyHttpExecute for MySQLExecuteProxy {
    async fn execute(
        &self,
        mut cx: RequestCx,
        db_conn: Arc<dyn AnyDatabaseConnection>,
    ) -> Result<HttpResponse, RequestError> {
        let MySQLExecuteProxy(mysql_execute) = self;

        let RequestCx { request_params, .. } = &mut cx;

        // Inject runtime parameters.
        let _config_guard = AppCx::acquire().config().read().await;

        request_params.insert(
            "users_target_table".to_compact_string(),
            ParamValue::Internal(
                _config_guard
                    .internal_params()
                    .users_target_table()
                    .to_owned(),
            ),
        );
        request_params.insert(
            "sessions_target_table".to_compact_string(),
            ParamValue::Internal(
                _config_guard
                    .internal_params()
                    .sessions_target_table()
                    .to_owned(),
            ),
        );
        request_params.insert(
            "roles_target_table".to_compact_string(),
            ParamValue::Internal(
                _config_guard
                    .internal_params()
                    .roles_target_table()
                    .to_owned(),
            ),
        );

        mysql_execute.execute(cx, db_conn).await
    }
}
