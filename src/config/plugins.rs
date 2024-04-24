use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    config::models::{extensions::modrinth::ModrinthData, model::ModelExtensions},
    downloader::hash::ChooseHash,
    errors::error::Result,
};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Plugin {
    // Откуда качаем
    #[serde(default)]
    pub source: Sources,
    // Версия
    #[serde(default)]
    pub version: Option<String>,
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
    Spigot, // bad api
    Hangar, // ?
    #[default]
    Modrinth, // Favorite
    CurseForge, // ?
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Channels {
    #[default]
    Release,
    Beta,
    Alpha,
}

impl Channels {
    pub async fn get_str(&self) -> &'static str {
        match self {
            Channels::Release => "release",
            Channels::Beta => "beta",
            Channels::Alpha => "alpha",
        }
    }
}

impl Plugin {
    pub async fn freeze(&self) -> bool {
        info!("Check freeze and force_update");
        if self.freeze && !self.force_update {
            return true;
        };
        false
    }

    pub async fn get_link(
        &self,
        name: &str,
        game_version: Option<&str>,
    ) -> Result<(String, ChooseHash, String)> {
        info!("Start to match provider of core");
        match self.source {
            Sources::Spigot => todo!(),
            Sources::Hangar => todo!(),
            Sources::Modrinth => ModrinthData::get_link(name, self, game_version).await,
            Sources::CurseForge => todo!(),
        }
    }
}
