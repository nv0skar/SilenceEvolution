// SilenceEvolution
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

/// Silence's endpoint test.
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
pub struct EndpointTest {
    #[serde(default = "default_name")]
    name: CompactString,

    endpoint_id: CompactString,

    #[serde(default)]
    description: Option<CompactString>,

    #[serde(default)]
    route_params: HashMap<CompactString, CompactString>,

    #[serde(default)]
    query_params: HashMap<CompactString, CompactString>,

    #[serde(default, skip_serializing_if = "should_skip_option")]
    body: Option<CompactString>,

    #[serde(default, skip_serializing_if = "should_skip_option")]
    response: Option<HashMap<CompactString, CompactString>>,
}

fn default_name() -> CompactString {
    format!("Test at {}", Utc::now().naive_utc()).into()
}

impl Default for EndpointTest {
    fn default() -> Self {
        Self {
            name: "My test".into(),
            endpoint_id: "MyEndpoint".into(),
            description: Some("Get contents of table.".into()),
            route_params: Default::default(),
            query_params: Default::default(),
            body: Default::default(),
            response: Default::default(),
        }
    }
}
