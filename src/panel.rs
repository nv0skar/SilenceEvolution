// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

/// TODO: add docs here.
#[derive(Clone, RustEmbed, Debug)]
#[folder = "./target/frontend"]
pub struct StaticFiles;

impl Service<RouterRequest> for StaticFiles {
    type Response = Response<String>;

    type Error = Infallible;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, cx: RouterRequest) -> Self::Future {
        let (request, _) = cx;

        Box::pin(async move {
            let route = request.uri().path().trim_matches('/').to_owned();

            let mut response = Response::builder()
                .header("Content-Type", "application/json; charset=utf-8")
                .header(
                    "Cache-Control",
                    format!(
                        "max-age={}",
                        (*RuntimeCx::acquire()
                            .build()
                            .read()
                            .await
                            .executor()
                            .http_cache_time()) as u32
                    ),
                );

            let body = match route.split('/').next() {
                Some(route_part) => match route_part {
                    "panel" | "internal_assets" => {
                        if let Some(file) = StaticFiles::get({
                            if route
                                .strip_prefix("panel")
                                .map(|part| part.trim_start_matches('/').is_empty())
                                .unwrap_or(false)
                            {
                                "panel/index.html"
                            } else {
                                &route
                            }
                        }) {
                            response = response.header("Content-Type", file.mime_type().unwrap());

                            let content = file.data();

                            Some(String::from_utf8(content).unwrap())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                None => None,
            };

            if let Some(body) = body {
                Ok(response.status(200).body(body).unwrap())
            } else {
                Ok(response
                    .status(404)
                    .body(
                        serde_json::to_string_pretty(&json!({
                            "error": "Resource not found."
                        }))
                        .unwrap(),
                    )
                    .unwrap())
            }
        })
    }
}
