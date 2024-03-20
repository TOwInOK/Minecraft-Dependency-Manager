use serde::{Deserialize, Serialize};
use crate::config::Versions;

#[derive(Deserialize, Serialize, Debug)]
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


#[derive(Deserialize, Serialize, Debug, Default)]
pub enum Sources {
    Bucket,
    Spigot,
    Hangar,
    #[default]
    Modrinth,
    CurseForge
}


#[derive(Deserialize, Serialize, Debug, Default)]
pub enum Channels {
    #[default]
    Stable,
    Beta,
    Alpha,
}

