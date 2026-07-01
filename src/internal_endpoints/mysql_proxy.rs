// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use execute::{mysql::*, *};

/// Proxies MySQL execute queries and injects internal params on runtime.
#[derive(Clone, PartialEq, Serialize, Deserialize, Getters, Display, Debug)]
#[display("SQL queries: {:?}", queries)]
#[getset(get = "pub")]
pub struct MySQLExecuteProxy {
    queries: CheapVec<CompactString>,
}

boxed_any!(MySQLExecuteProxy);

impl MySQLExecuteProxy {
    pub fn new(queries: CompactString) -> Self {
        let queries = queries
            .split(';')
            .map(|query| query.into())
            .collect::<CheapVec<CompactString>>();

        Self { queries }
    }

    /// This will enable Silence to execute multiple MySQL queries until Waveless' MySQL implements this.
    pub fn new_many(queries: CheapVec<CompactString>) -> Self {
        Self { queries }
    }
}

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

        let mut last_res: Result<ExecuteOutput, RequestError> =
            Err(RequestError::Other(anyhow!("No query was executed.")));

        for query in self.queries.to_owned() {
            let mysql_execute = Arc::new(MySQLExecute::new(query.to_owned()));

            last_res = mysql_execute
                .execute(method, db_conn.to_owned(), input.to_owned())
                .await;
        }

        last_res
    }
}
