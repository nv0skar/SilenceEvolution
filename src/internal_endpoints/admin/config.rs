// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use config::*;

use databases::*;
use http_execute::{request_cx::*, *};

#[derive(Clone, Serialize, Deserialize, Getters, Display, Debug)]
#[display("Endpoint manager.")]
pub struct ConfigManager;

boxed_any!(ConfigManager);

/// TODO: add docs here.
#[typetag::serde(name = "ConfigManager")]
#[async_trait]
impl AnyHttpExecute for ConfigManager {
    async fn execute(
        &self,
        cx: RequestCx,
        _: Arc<dyn AnyDatabaseConnection>,
    ) -> Result<HttpResponse, RequestError> {
        let RequestCx {
            request, method, ..
        } = cx;

        match method {
            HttpMethod::Get => {
                let config = AppCx::acquire().config().read().await.to_owned();

                let res = serde_json::to_value(config).map_err(|err| {
                    RequestError::Other(anyhow!("Cannot serialize endpoints. {}", err))
                })?;

                Ok(HttpResponse::new(None, Some(BodyValue::Json(res))))
            }
            HttpMethod::Put => {
                {
                    let mut config_guard = AppCx::acquire().config().write().await;

                    let config_patch = serde_json::from_slice::<ConfigPatch>(
                        &request
                            .collect()
                            .await
                            .map_err(|err| {
                                RequestError::Expected(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    format!("Cannot get request's body. {}", err).into(),
                                )
                            })?
                            .to_bytes()
                            .to_vec(),
                    )
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

                Ok(HttpResponse::new(None, None))
            }
            _ => unreachable!(),
        }
    }
}
