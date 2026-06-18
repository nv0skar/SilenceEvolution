// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use config::*;

use databases::*;
use execute::*;

#[derive(Clone, Serialize, Deserialize, Getters, Display, Debug)]
#[display("Endpoint manager.")]
pub struct ConfigManager;

boxed_any!(ConfigManager);

/// TODO: add docs here.
#[typetag::serde(name = "ConfigManager")]
#[async_trait]
impl AnyExecute for ConfigManager {
    async fn execute(
        &self,
        method: HttpMethod,
        _: Arc<dyn AnyDatabaseConnection>,
        input: ExecuteInput,
    ) -> Result<ExecuteOutput, RequestError> {
        match method {
            HttpMethod::Get => {
                let config = AppCx::acquire().config().read().await.to_owned();

                let res = serde_json::to_value(config).map_err(|err| {
                    RequestError::Other(anyhow!("Cannot serialize endpoints. {}", err))
                })?;

                Ok(ExecuteOutput::Json(None, res))
            }
            HttpMethod::Put => {
                {
                    let mut config_guard = AppCx::acquire().config().write().await;

                    let config_patch = serde_json::from_slice::<ConfigPatch>(input.value())
                        .map_err(|err| {
                            RequestError::Expected(
                                StatusCode::BAD_REQUEST,
                                format!("Cannot deserialize config's patch. {}", err)
                                    .to_compact_string(),
                            )
                        })?;

                    config_guard.apply(config_patch);
                }

                AppCx::acquire().set_config().await?;

                Ok(ExecuteOutput::Json(None, json!({})))
            }
            _ => unreachable!(),
        }
    }
}
