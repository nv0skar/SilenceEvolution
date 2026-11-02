// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use backend::session_manager::*;

use auth::*;
use databases::*;
use http_executor::{request_cx::*, *};

/// Enables admins to issue new tokens for an arbitrary user.
#[derive(Clone, Serialize, Deserialize, BoxedAny, Getters, Display, Debug)]
#[display("Token issuer.")]
pub struct TokenIssuer;

impl AnyExt for TokenIssuer {
    fn name(&self) -> &str {
        "silence_token_issuer"
    }
}

/// TODO: add docs here.
#[typetag::serde(name = "TestsManager")]
#[async_trait]
impl AnyHttpExecutor for TokenIssuer {
    async fn execute(&self, cx: PipelineCx, _db_conns: DbConns) -> PipelineResult {
        let PipelineCx {
            mut request,
            response,
        } = cx;

        let RequestCx { request_params, .. } = &mut request;

        let mut response = response.unwrap_or_default();

        let ParamValue::Client(Some(user_id)) =
            request_params.get("id").ok_or(RequestError::Expected(
                StatusCode::BAD_REQUEST,
                "The `user_id` wasn't provided.".into(),
            ))?
        else {
            unreachable!()
        };

        // Issue the new token.
        let token = InMemoryToken
            .new(_db_conns, user_id.parse::<UserId>().unwrap())
            .await?;

        *response.body_mut() = Some(BodyValue::Json(json!({"token": token})));

        Ok((
            PipelineCx {
                request,
                response: Some(response),
            },
            PipelineAction::Continue(None),
        ))
    }
}
