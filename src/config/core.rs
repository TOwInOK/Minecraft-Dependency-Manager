use crate::config::Versions;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Core {
    // Ядро
    #[serde(default)]
    pub provider: Provider,
    // Версия ядра
    #[serde(default)]
    pub version: Versions,
    // Версия билда ядра
    #[serde(default)]
    pub build: String,
    // Приостановить обновление
    #[serde(default)]
    pub freeze: bool,
    // Нужно обновить
    #[serde(default)]
    pub force_update: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    #[default]
    Vanilla,
    Paper,
    Folia,
    Purpur,
    Fabric,
    Forge,
    NeoForge,
}
