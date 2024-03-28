use crate::config::Versions;
use log::info;
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
    pub async fn get_name(&self) -> String {
        match self {
            Provider::Vanilla => "Vanilla".to_owned(),
            Provider::Paper => "Paper".to_owned(),
            Provider::Folia => "Folia".to_owned(),
            Provider::Purpur => "Purpur".to_owned(),
            Provider::Fabric => "Fabric".to_owned(),
            Provider::Forge => "Forge".to_owned(),
            Provider::NeoForge => "NeoForge".to_owned(),
            Provider::Waterfall => "Waterfall".to_owned(),
            Provider::Velocity => "Velocity".to_owned(),
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
