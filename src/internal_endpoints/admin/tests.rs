// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use http_execute::{request_cx::*, *};

#[derive(Clone, Serialize, Deserialize, Getters, Display, Debug)]
#[display("Tests manager.")]
pub struct EndpointTestsManager;

boxed_any!(EndpointTestsManager);

/// TODO: add docs here.
#[typetag::serde(name = "TestsManager")]
#[async_trait]
impl AnyHttpExecute for EndpointTestsManager {
    async fn execute(
        &self,
        cx: RequestCx,
        _: Arc<dyn AnyDatabaseConnection>,
    ) -> Result<HttpResponse, RequestError> {
        let RequestCx {
            method,
            request_params,
            request,
            ..
        } = cx;

        match method {
            HttpMethod::Get => {
                let test_name = request_params.get("test_name").cloned();

                match test_name {
                    Some(ParamValue::Client(Some(mut name))) => {
                        name = name.replace("%20", " ").into();

                        let endpoint_test = AppCx::acquire()
                            .get_test(name.to_owned())
                            .await?
                            .ok_or(RequestError::Expected(
                                StatusCode::BAD_REQUEST,
                                format!("Cannot find test with name `{}`", name)
                                    .to_compact_string(),
                            ))?;

                        let res = serde_json::to_value(endpoint_test).map_err(|err| {
                            RequestError::Other(anyhow!("Cannot serialize test. {}", err))
                        })?;

                        Ok(HttpResponse::new(None, Some(BodyValue::Json(res))))
                    }
                    None => {
                        let endpoint_tests = AppCx::acquire().get_tests().await?;

                        let res = serde_json::to_value(endpoint_tests).map_err(|err| {
                            RequestError::Other(anyhow!("Cannot serialize tests. {}", err))
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

                let endpoint_test =
                    serde_json::from_slice::<EndpointTest>(&req_body).map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("Cannot deserialize test. {}", err).to_compact_string(),
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
                    .add_test(target_file, endpoint_test)
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

                let endpoint_test_name = match request_params.get("test_name") {
                    Some(ParamValue::Client(Some(name))) => name.to_owned(),
                    _ => Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        format!("Test's name to delete was not specified.").to_compact_string(),
                    ))?,
                }
                .replace("%20", " ")
                .into();

                let endpoint_test_patch = serde_json::from_slice::<EndpointTestPatch>(req_body)
                    .map_err(|err| {
                        RequestError::Expected(
                            StatusCode::BAD_REQUEST,
                            format!("Cannot deserialize test's patch. {}", err).to_compact_string(),
                        )
                    })?;

                AppCx::acquire()
                    .set_test(endpoint_test_name, endpoint_test_patch)
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
                let endpoint_test_name = match request_params.get("test_name") {
                    Some(ParamValue::Client(Some(id))) => id.to_owned(),
                    _ => Err(RequestError::Expected(
                        StatusCode::BAD_REQUEST,
                        format!("Test's name to delete was not specified.").to_compact_string(),
                    ))?,
                }
                .replace("%20", " ")
                .into();

                AppCx::acquire()
                    .delete_test(endpoint_test_name)
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
            _ => unreachable!(),
        }
    }
}
