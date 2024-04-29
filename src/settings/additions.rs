use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Additions {
    // git link
    #[serde(default)]
    config_plugins_from: Option<String>,
    // git key
    #[serde(default)]
    key: Option<String>,
}