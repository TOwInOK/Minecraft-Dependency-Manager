use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Additions {
    // git link
    #[serde(rename = "configPluguinsFrom")]
    config_plugins_from: String,
    // git key
    key: String,
}