// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use execute::{mysql::*, *};

/// Proxies MySQL execute queries and injects internal params on runtime.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Display, Debug)]
#[display("SQL query: {:?}", query)]
#[getset(get = "pub")]
pub struct MySQLExecuteProxy {
    query: CompactString,
}

boxed_any!(MySQLExecuteProxy);

#[typetag::serde(name = "MySQLProxy")]
#[async_trait]
impl AnyExecute for MySQLExecuteProxy {
    async fn execute(
        &self,
        method: HttpMethod,
        db_conn: Arc<dyn AnyDatabaseConnection>,
        mut input: ExecuteInput,
    ) -> Result<ExecuteOutput, RequestError> {
        // Inject runtime parameters.
        let _config_guard = AppCx::acquire().config().read().await;
        input.params_mut().insert(
            "user_id_row".to_compact_string(),
            ExecuteParamValue::Internal(_config_guard.internal_params().user_id().to_owned()),
        );
        input.params_mut().insert(
            "users_target_table".to_compact_string(),
            ExecuteParamValue::Internal(
                _config_guard
                    .internal_params()
                    .users_target_table()
                    .to_owned(),
            ),
        );
        input.params_mut().insert(
            "sessions_target_table".to_compact_string(),
            ExecuteParamValue::Internal(
                _config_guard
                    .internal_params()
                    .sessions_target_table()
                    .to_owned(),
            ),
        );
        input.params_mut().insert(
            "roles_target_table".to_compact_string(),
            ExecuteParamValue::Internal(
                _config_guard
                    .internal_params()
                    .roles_target_table()
                    .to_owned(),
            ),
        );

        let mysql_execute = Arc::new(MySQLExecute::new(self.query.to_owned()));

        mysql_execute.execute(method, db_conn, input).await
    }
}
