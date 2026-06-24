// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use mysql_proxy::*;

use execute::mysql::*;

/// Silence's endpoint.
/// TODO: add documentation.
#[derive(
    Clone,
    PartialEq,
    Constructor,
    Builder,
    Serialize,
    Deserialize,
    Getters,
    MutGetters,
    Patch,
    Debug,
)]
#[patch(attribute(derive(Clone, PartialEq, Constructor, Builder, Serialize, Deserialize, Debug)))]
#[builder(default, pattern = "mutable", setter(strip_option))]
#[getset(get = "pub", get_mut = "pub")]
pub struct SimpleEndpoint {
    id: CompactString,

    route: CompactString,

    #[serde(default)]
    description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "should_skip_option")]
    version: Option<CompactString>,

    method: HttpMethod,

    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    query_params: CheapVec<CompactString, 0>,

    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    body_params: CheapVec<CompactString, 0>,

    query: Option<CompactString>,

    #[serde(default)]
    require_auth: bool,

    #[serde(default, skip_serializing_if = "should_skip_cheapvec")]
    allowed_roles: CheapVec<CompactString, 0>,

    #[serde(default)]
    inject_user_id: bool,

    #[serde(default, skip_serializing_if = "auto_generated_skip")]
    auto_generated: bool,
}

fn auto_generated_skip(value: &bool) -> bool {
    should_skip(&(!*value))
}

impl Default for SimpleEndpoint {
    fn default() -> Self {
        Self {
            id: "Example".to_compact_string(),
            route: "my_endpoint".to_compact_string(),
            version: Some("v1".to_compact_string()),
            description: None,
            method: HttpMethod::Get,
            query_params: CheapVec::new(),
            body_params: CheapVec::new(),
            query: Some("SELECT * FROM example".to_compact_string()),
            require_auth: false,
            allowed_roles: CheapVec::new(),
            inject_user_id: false,
            auto_generated: false,
        }
    }
}

impl From<waveless_commons::endpoint::Endpoint> for SimpleEndpoint {
    fn from(endpoint: waveless_commons::endpoint::Endpoint) -> Self {
        SimpleEndpoint::from(&endpoint)
    }
}

impl<'a> From<&'a waveless_commons::endpoint::Endpoint> for SimpleEndpoint {
    fn from(endpoint: &'a waveless_commons::endpoint::Endpoint) -> Self {
        let query = {
            if let Some(query) = endpoint
                .execute()
                .to_owned()
                .map(|execute| execute.into_arc_any().downcast::<MySQLExecute>().ok())
                .flatten()
                .map(|mysql_execute| mysql_execute.query().to_compact_string())
            {
                Some(query)
            } else if let Some(query) = endpoint
                .execute()
                .to_owned()
                .map(|execute| execute.into_arc_any().downcast::<MySQLExecuteProxy>().ok())
                .flatten()
                .map(|mysql_execute| mysql_execute.queries().join("; ").to_compact_string())
            {
                Some(query)
            } else {
                None
            }
        };

        SimpleEndpoint {
            id: endpoint.id().to_owned(),
            route: endpoint.route().to_owned(),
            version: endpoint.version().to_owned(),
            description: endpoint.description().to_owned(),
            method: endpoint.method().to_owned(),
            query_params: endpoint.query_params().to_owned(),
            body_params: endpoint.body_params().to_owned(),
            query: query,
            require_auth: *endpoint.require_auth(),
            allowed_roles: endpoint.allowed_roles().to_owned(),
            inject_user_id: *endpoint.inject_user_id(),
            auto_generated: *endpoint.auto_generated(),
        }
    }
}

impl Into<waveless_commons::endpoint::Endpoint> for SimpleEndpoint {
    fn into(self) -> waveless_commons::endpoint::Endpoint {
        (&self).into()
    }
}

impl<'a> Into<waveless_commons::endpoint::Endpoint> for &'a SimpleEndpoint {
    fn into(self) -> waveless_commons::endpoint::Endpoint {
        let mut endpoint_builder = &mut EndpointBuilder::default();

        endpoint_builder = endpoint_builder
            .id(self.id.to_owned())
            .route(self.route.to_owned())
            .method(self.method.to_owned())
            .query_params(self.query_params.to_owned())
            .body_params(self.body_params.to_owned())
            .require_auth(*self.require_auth())
            .allowed_roles(self.allowed_roles.to_owned())
            .inject_user_id(self.inject_user_id);

        if let Some(version) = self.version.to_owned() {
            endpoint_builder = endpoint_builder.version(version);
        };

        if let Some(query) = self.query.to_owned() {
            endpoint_builder = endpoint_builder.execute(Arc::new(MySQLExecuteProxy::new(query)));
        }

        if let Some(description) = self.description.to_owned() {
            endpoint_builder = endpoint_builder.description(description);
        };

        endpoint_builder.build().unwrap()
    }
}
