use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Additions {
    // git link
    #[serde(default)]
    config_plugins_from: Option<String>,
    // git key
    #[serde(default)]
    key: Option<String>,
    // Paths
    #[serde(default = "core")]
    pub path_to_core: String,
    #[serde(default = "mods")]
    pub path_to_mods: String,
    #[serde(default = "plugins")]
    pub path_to_plugins: String,
    #[serde(default = "configs")]
    pub path_to_configs: String,
}

fn core() -> String {
    "./core".to_string()
}
fn mods() -> String {
    "./mods".to_string()
}
fn plugins() -> String {
    "./plugins".to_string()
}
fn configs() -> String {
    "./configs".to_string()
}
