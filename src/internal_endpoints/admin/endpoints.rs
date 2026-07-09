// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use http_execute::{request_cx::*, *};

#[derive(Clone, Serialize, Deserialize, Getters, Display, Debug)]
#[display("Endpoint manager.")]
pub struct EndpointsManager;

boxed_any!(EndpointsManager);

/// TODO: add docs here.
#[typetag::serde(name = "EndpointManager")]
#[async_trait]
impl AnyHttpExecute for EndpointsManager {
    async fn execute(
        &self,
        cx: RequestCx,
        _: Arc<dyn AnyDatabaseConnection>,
    ) -> Result<HttpResponse, RequestError> {
        let RequestCx {
            request,
            method,
            request_params,
            ..
        } = cx;

        match method {
            HttpMethod::Get => {
                let simple_endpoint_id = request_params.get("endpoint_id");

                match simple_endpoint_id {
                    Some(ParamValue::Client(Some(id))) => {
                        let simple_endpoints_by_file = AppCx::acquire().get_endpoints().await?;

                        let simple_endpoint = simple_endpoints_by_file
                            .iter()
                            .flat_map(|(_, endpoints)| endpoints)
                            .find(|endpoint| endpoint.id() == id)
                            .ok_or(RequestError::Expected(
                                StatusCode::BAD_REQUEST,
                                format!("Cannot find endpoint with id {}", id).to_compact_string(),
                            ))?;

                        let serialized_endpoint =
                            serde_json::to_value(simple_endpoint).map_err(|err| {
                                RequestError::Other(anyhow!("Cannot serialize endpoint. {}", err))
                            })?;

                        Ok(HttpResponse::new(
                            None,
                            Some(BodyValue::Json(serialized_endpoint)),
                        ))
                    }
                    None => {
                        let simple_endpoints_by_file = AppCx::acquire().get_endpoints().await?;

                        let res =
                            serde_json::to_value(simple_endpoints_by_file).map_err(|err| {
                                RequestError::Other(anyhow!("Cannot serialize endpoints. {}", err))
                            })?;

                        Ok(HttpResponse::new(None, Some(BodyValue::Json(res))))
                    }
                    _ => unreachable!(),
                }
            }
            HttpMethod::Post => {
                let req_body = Bytes::from_vec(
                    request
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
                );

                let simple_endpoint =
                    serde_json::from_slice::<SimpleEndpoint>(&req_body).map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("Cannot deserialize endpoint. {}", err).to_compact_string(),
                        )
                    })?;

                let target_file = match request_params.get("target_file") {
                    Some(ParamValue::Client(Some(target_file))) => target_file.to_owned(),
                    _ => "default.json".to_compact_string(),
                };

                if !target_file.ends_with(".json") {
                    Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        format!("Target file {} doesn't end with `.json`.", target_file)
                            .to_compact_string(),
                    ))?;
                }

                AppCx::acquire()
                    .add_endpoint(target_file, simple_endpoint)
                    .await
                    .map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("{}. Operation might have been partially completed.", err)
                                .to_compact_string(),
                        )
                    })?;

                Ok(HttpResponse::new(None, None))
            }
            HttpMethod::Put => {
                let req_body = &request
                    .collect()
                    .await
                    .map_err(|err| {
                        RequestError::Expected(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Cannot get request's body. {}", err).into(),
                        )
                    })?
                    .to_bytes()
                    .to_vec();

                let simple_endpoint_id = match request_params.get("endpoint_id") {
                    Some(ParamValue::Client(Some(id))) => id.to_owned(),
                    _ => Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        format!("Endpoint's id to delete was not specified.").to_compact_string(),
                    ))?,
                };

                let simple_endpoint_patch = serde_json::from_slice::<SimpleEndpointPatch>(req_body)
                    .map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("Cannot deserialize endpoint's patch. {}", err)
                                .to_compact_string(),
                        )
                    })?;

                AppCx::acquire()
                    .set_endpoint(simple_endpoint_id, simple_endpoint_patch)
                    .await
                    .map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("{}. Operation might have been partially completed.", err)
                                .to_compact_string(),
                        )
                    })?;

                Ok(HttpResponse::new(None, None))
            }
            HttpMethod::Delete => {
                let simple_endpoint_id = match request_params.get("id") {
                    Some(ParamValue::Client(Some(id))) => id.to_owned(),
                    _ => Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        format!("Endpoint id to delete was not specified.").to_compact_string(),
                    ))?,
                };

                AppCx::acquire()
                    .delete_endpoint(simple_endpoint_id)
                    .await
                    .map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("{}. Operation might have been partially completed.", err)
                                .to_compact_string(),
                        )
                    })?;

                Ok(HttpResponse::new(None, None))
            }
            HttpMethod::Unknown => unreachable!(),
        }
    }
}
