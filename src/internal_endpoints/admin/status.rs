// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use databases::*;
use http_executor::{request_cx::*, *};

/// Executor to announce the current status of the server.
#[derive(Clone, Serialize, Deserialize, BoxedAny, Getters, Display, Debug)]
#[display("Silence's status.")]
pub struct Status(pub(crate) StatusOperation);

#[derive(Clone, Serialize, Deserialize, BoxedAny, Display, Debug)]
pub enum StatusOperation {
    Info,
    Health,
}

impl AnyExt for Status {
    fn name(&self) -> &str {
        "silence_status"
    }
}

/// TODO: add docs here.
#[typetag::serde(name = "Status")]
#[async_trait]
impl AnyHttpExecutor for Status {
    async fn execute(&self, cx: PipelineCx, _db_conns: DbConns) -> PipelineResult {
        let PipelineCx {
            mut request,
            response,
        } = cx;

        let RequestCx { .. } = &mut request;

        let mut response = response.unwrap_or_default();

        *response.body_mut() = None; // Empty the response body set by previous execution steps.

        let Status(operation) = self;

        match operation {
            StatusOperation::Info => {
                let app_cx = AppCx::acquire();

                *response.body_mut() = Some(BodyValue::Json(json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "project": app_cx.workspace_root().into_iter().last().unwrap().display().to_string(),
                    "path": app_cx.workspace_root()
                })));
            }
            StatusOperation::Health => {
                todo!()
            }
        }

        Ok((
            PipelineCx {
                request,
                response: Some(response),
            },
            PipelineAction::Continue(None),
        ))
    }
}
