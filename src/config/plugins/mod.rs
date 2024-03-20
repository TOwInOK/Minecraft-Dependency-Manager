mod models;

use serde::{Deserialize, Serialize};
use crate::config::Versions;

#[derive(Deserialize, Serialize, Debug)]
pub struct Plugin {
    // Откуда качаем
    #[serde(default)]
    source: Sources,
    // Версия
    #[serde(default)]
    version: Versions,
    // Стабильная, Альфа, Бета
    #[serde(default)]
    channel: Channels,
    // Приостановить обновление
    #[serde(default)]
    freeze: bool,
    // Нужно обновить
    #[serde(default)]
    force_update: bool,
}

#[derive(Deserialize, Serialize, Debug)]
enum Sources {
    Bucket,
    Spigot,
    Hangar,
    Modrinth,
    CurseForge
}

impl Default for Sources {
    fn default() -> Self {
        Self::Modrinth
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum Channels {
    Stable,
    Beta,
    Alpha,
}

impl Default for Channels {
    fn default() -> Self {
        Self::Stable
    }
}
