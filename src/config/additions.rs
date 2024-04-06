use log::warn;
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
    #[serde(default = "lock")]
    pub path_to_lock: String,
    #[serde(default = "configs")]
    pub path_to_configs: String,
}

fn core() -> String {
    warn!("We use default core path!");
    "core".to_string()
}
fn mods() -> String {
    warn!("We use default mods path!");
    "mods".to_string()
}
fn plugins() -> String {
    warn!("We use default plugins path!");
    "plugins".to_string()
}
fn configs() -> String {
    warn!("We use default config path!");
    "configs.toml".to_string()
}
fn lock() -> String {
    warn!("We use default lock path!");
    "config.lock".to_string()
}
