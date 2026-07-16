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

    target_endpoint_id: CompactString,

    #[serde(default)]
    #[patch(skip_wrap)]
    description: Option<CompactString>,

    #[serde(default, flatten)]
    #[patch(attribute(serde(default, flatten)), skip_wrap)]
    metadata: Option<HashMap<CompactString, serde_json::Value>>,
}

fn default_name() -> CompactString {
    format!("Test at {}", Utc::now().naive_utc()).into()
}

impl Default for EndpointTest {
    fn default() -> Self {
        Self {
            name: "My test".into(),
            target_endpoint_id: "MyEndpoint".into(),
            description: Some("Get contents of table.".into()),
            metadata: Default::default(),
        }
    }
}
