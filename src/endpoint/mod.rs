// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

pub mod internal;

use crate::*;

use waveless_commons::{
    endpoint::{EndpointBuilder, HttpMethod},
    *,
};

use execute::mysql::*;

/// Silence's endpoint.
#[derive(Clone, PartialEq, Constructor, Serialize, Deserialize, Getters, Debug)]
#[getset(get = "pub")]
pub struct Endpoint {
    id: CompactString,
    route: CompactString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<CompactString>,
    method: HttpMethod,
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    query_params: CheapVec<CompactString, 0>,
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    body_params: CheapVec<CompactString, 0>,
    query: CompactString,
    #[serde(default, skip_serializing_if = "should_skip")]
    require_auth: bool,
    #[serde(default, skip_serializing_if = "CheapVec::is_empty")]
    allowed_roles: CheapVec<CompactString, 0>,
    #[serde(default, skip_serializing_if = "should_skip")]
    inject_user_id: bool,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            id: "Example".to_compact_string(),
            route: "my_endpoint".to_compact_string(),
            description: None,
            method: HttpMethod::Get,
            query_params: CheapVec::new(),
            body_params: CheapVec::new(),
            query: "SELECT * FROM example".to_compact_string(),
            require_auth: false,
            allowed_roles: CheapVec::new(),
            inject_user_id: false,
        }
    }
}

impl Into<waveless_commons::endpoint::Endpoint> for Endpoint {
    fn into(self) -> waveless_commons::endpoint::Endpoint {
        let mut endpoint_builder = &mut EndpointBuilder::default();

        endpoint_builder = endpoint_builder
            .id(self.id.to_owned())
            .route(self.route.to_owned())
            .method(self.method.to_owned())
            .query_params(self.query_params.to_owned())
            .body_params(self.body_params.to_owned())
            .execute(Arc::new(MySQLExecute::new(self.query.to_owned())))
            .require_auth(*self.require_auth())
            .inject_user_id(self.inject_user_id);

        if let Some(description) = self.description {
            endpoint_builder = endpoint_builder.description(description);
        };

        endpoint_builder.build().unwrap()
    }
}
