use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Additions {
    // git link
    #[serde(rename = "configPluguinsFrom")]
    config_plugins_from: String,
    // git key
    key: String,
}

impl Default for Additions {
    fn default() -> Self {
        Self { config_plugins_from: Default::default(), key: Default::default() }
    }
}