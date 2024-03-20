use crate::config::Versions;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Core {
    // Ядро
    #[serde(default)]
    provider: Provider,
    // Версия ядра
    #[serde(default)]
    version: Versions,
    // Приостановить обновление
    #[serde(default)]
    freeze: bool,
    // Нужно обновить
    #[serde(default)]
    force_update: bool,
}

impl Default for Core {
    fn default() -> Self {
        Self { provider: Default::default(), version: Default::default(), freeze: Default::default(), force_update: Default::default() }
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum Provider {
    Vanilla,
    Bucket,
    Spigot,
    Paper,
    Purpur,
    Fabric,
    Forge,
    NeoForge,
}

impl Default for Provider {
    fn default() -> Self {
        Self::Vanilla
    }
}
