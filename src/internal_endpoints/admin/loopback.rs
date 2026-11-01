// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use http_executor::{request_cx::*, *};

/// Returns whether the server is listening on a loopback address.
#[derive(Clone, Serialize, Deserialize, BoxedAny, Getters, Display, Debug)]
#[display("Endpoint manager.")]
pub struct IsLoopback;

impl AnyExt for IsLoopback {
    fn name(&self) -> &str {
        "silence_is_loopback"
    }
}

/// TODO: add docs here.
#[typetag::serde(name = "IsLoopback")]
#[async_trait]
impl AnyHttpExecutor for IsLoopback {
    async fn execute(&self, cx: PipelineCx, _db_conns: DbConns) -> PipelineResult {
        let PipelineCx {
            mut request,
            response,
        } = cx;

        let RequestCx { .. } = &mut request;

        let mut response = response.unwrap_or_default();

        let loopback = AppCx::acquire()
            .config()
            .read()
            .await
            .listening_addr()
            .map(|listening_addr| listening_addr.ip().is_loopback())
            .unwrap_or_default();

        *response.body_mut() = Some(BodyValue::Json(json!({"loopback": loopback})));

        Ok((
            PipelineCx {
                request,
                response: Some(response),
            },
            PipelineAction::Continue(None),
        ))
    }
}
