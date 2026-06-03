// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use crate::endpoint::*;
use databases::*;
use execute::*;

#[derive(Clone, Serialize, Deserialize, Getters, Display, Debug)]
#[display("Endpoint manager.")]
pub struct EndpointManager;

boxed_any!(EndpointManager);

#[typetag::serde(name = "EndpointManager")]
#[async_trait]
impl AnyExecute for EndpointManager {
    async fn execute(
        &self,
        method: HttpMethod,
        _: Arc<dyn AnyDatabaseConnection>,
        input: ExecuteInput,
    ) -> Result<ExecuteOutput, RequestError> {
        let runtime_build = RuntimeCx::acquire().build().read().await;

        match method {
            HttpMethod::Get => {
                let id = input.params().get("id");

                match id {
                    Some(ExecuteParamValue::Client(Some(id))) => {
                        let endpoint: UserEndpoint = runtime_build
                            .endpoints()
                            .inner()
                            .iter()
                            .find(|endpoint| endpoint.id() == id)
                            .ok_or(RequestError::Expected(
                                StatusCode::BAD_REQUEST,
                                format!("Cannot find endpoint with id {}", id).to_compact_string(),
                            ))?
                            .to_owned()
                            .into();

                        BINARY_MODE.set(true);

                        let serialized_endpoint =
                            serde_json::to_value(endpoint).map_err(|err| {
                                RequestError::Other(anyhow!("Cannot serialize endpoint. {}", err))
                            })?;

                        BINARY_MODE.set(false);

                        Ok(ExecuteOutput::Json(None, serialized_endpoint))
                    }
                    None => {
                        let endpoints = runtime_build
                            .endpoints()
                            .inner()
                            .iter()
                            .map(|endpoint| endpoint.into())
                            .collect::<CheapVec<UserEndpoint>>();

                        BINARY_MODE.set(true);

                        let serialized_endpoints =
                            serde_json::to_value(endpoints).map_err(|err| {
                                RequestError::Other(anyhow!("Cannot serialize endpoint. {}", err))
                            })?;

                        BINARY_MODE.set(false);

                        Ok(ExecuteOutput::Json(None, serialized_endpoints))
                    }
                    _ => unreachable!(),
                }
            }
            HttpMethod::Post => {
                let app_cx = AppCx::acquire();

                let endpoint =
                    serde_json::from_slice::<UserEndpoint>(input.value()).map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("Cannot deserialize endpoint. {}", err).to_compact_string(),
                        )
                    })?;

                AppCx::add_endpoint_into(
                    app_cx.user_endpoints().write().await,
                    "default.json".to_compact_string(),
                    endpoint,
                )?;

                Ok(ExecuteOutput::Json(None, json!({})))
            }
            HttpMethod::Put => todo!(),
            HttpMethod::Delete => todo!(),
            HttpMethod::Unknown => unreachable!(),
        }
    }
}
