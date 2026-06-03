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
/// TODO: add documentation.
#[derive(Clone, PartialEq, Constructor, Builder, Serialize, Deserialize, Getters, Debug)]
#[builder(default, pattern = "mutable", setter(strip_option))]
#[getset(get = "pub")]
pub struct UserEndpoint {
    id: CompactString,

    route: CompactString,

    #[serde(default, skip_serializing_if = "should_skip_option")]
    description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "should_skip_option")]
    version: Option<CompactString>,

    method: HttpMethod,

    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    query_params: CheapVec<CompactString, 0>,

    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    body_params: CheapVec<CompactString, 0>,

    query: CompactString,

    #[serde(default, skip_serializing_if = "should_skip")]
    require_auth: bool,

    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    allowed_roles: CheapVec<CompactString, 0>,

    #[serde(default, skip_serializing_if = "should_skip")]
    inject_user_id: bool,
}

impl Default for UserEndpoint {
    fn default() -> Self {
        Self {
            id: "Example".to_compact_string(),
            route: "my_endpoint".to_compact_string(),
            version: Some("v1".to_compact_string()),
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

impl From<waveless_commons::endpoint::Endpoint> for UserEndpoint {
    fn from(endpoint: waveless_commons::endpoint::Endpoint) -> Self {
        UserEndpoint::from(&endpoint)
    }
}

impl<'a> From<&'a waveless_commons::endpoint::Endpoint> for UserEndpoint {
    fn from(endpoint: &'a waveless_commons::endpoint::Endpoint) -> Self {
        let query = match endpoint
            .execute()
            .to_owned()
            .map(|execute| execute.into_arc_any().downcast::<MySQLExecute>().ok())
            .flatten()
        {
            Some(mysql_execute) => mysql_execute.query().to_compact_string(),
            None => "Internal".to_compact_string(),
        };

        UserEndpoint {
            id: endpoint.id().to_owned(),
            route: endpoint.route().to_owned(),
            version: Some("v1".to_compact_string()),
            description: endpoint.description().to_owned(),
            method: endpoint.method().to_owned(),
            query_params: endpoint.query_params().to_owned(),
            body_params: endpoint.body_params().to_owned(),
            query: query,
            require_auth: *endpoint.require_auth(),
            allowed_roles: endpoint.allowed_roles().to_owned(),
            inject_user_id: *endpoint.inject_user_id(),
        }
    }
}

impl Into<waveless_commons::endpoint::Endpoint> for UserEndpoint {
    fn into(self) -> waveless_commons::endpoint::Endpoint {
        (&self).into()
    }
}

impl<'a> Into<waveless_commons::endpoint::Endpoint> for &'a UserEndpoint {
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

        if let Some(version) = self.version.to_owned() {
            endpoint_builder = endpoint_builder.version(version);
        };

        if let Some(description) = self.description.to_owned() {
            endpoint_builder = endpoint_builder.description(description);
        };

        endpoint_builder.build().unwrap()
    }
}
