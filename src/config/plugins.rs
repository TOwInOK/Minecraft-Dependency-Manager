use crate::config::Versions;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Plugin {
    // Откуда качаем
    #[serde(default)]
    pub source: Sources,
    // Версия
    #[serde(default)]
    pub version: Versions,
    // Стабильная, Альфа, Бета
    #[serde(default)]
    pub channel: Channels,
    // Приостановить обновление
    #[serde(default)]
    pub freeze: bool,
    // Нужно обновить
    #[serde(default)]
    pub force_update: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Sources {
    Bukkit,
    Spigot,
    Hangar,
    #[default]
    Modrinth,
    CurseForge,
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Channels {
    #[default]
    Stable,
    Beta,
    Alpha,
}
