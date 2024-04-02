use log::info;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Core {
    // Ядро
    #[serde(default)]
    pub provider: Provider,
    // Версия ядра
    #[serde(default)]
    pub version: Option<String>,
    // Версия билда ядра
    #[serde(default)]
    pub build: Option<String>,
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
    Vanilla, // done
    Paper,  // done
    Folia,  // done
    Purpur, // in work, good api
    Fabric, // in work, api with out hash
    //https://meta.fabricmc.net/v2/versions/game <- version check /v2/versions/intermediary give only stable
    // or https://meta.fabricmc.net/v1/versions/game/1.14.4. Если нет версии, ответ пуст.
    Forge,     //no api
    NeoForge,  //worst api
    Waterfall, // done
    Velocity,  // done
}

impl Provider {
    pub async fn get_name(&self) -> &'static str {
        match self {
            Provider::Vanilla => "vanilla",
            Provider::Paper => "paper",
            Provider::Folia => "folia",
            Provider::Purpur => "purpur",
            Provider::Fabric => "fabric",
            Provider::Forge => "forge",
            Provider::NeoForge => "neoforge",
            Provider::Waterfall => "waterfall",
            Provider::Velocity => "velocity",
        }
    }
}

impl Core {
    pub async fn freeze(&self) -> bool {
        info!("Check freeze and force_update");
        if self.freeze && !self.force_update {
            info!("Core has iced");
            return true;
        };
        false
    }
}
