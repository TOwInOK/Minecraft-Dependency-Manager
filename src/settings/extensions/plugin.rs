use serde::{Deserialize, Serialize};

use crate::errors::error::Result;
use crate::models::extensions::modrinth::ModrinthData;
use crate::tr::hash::ChooseHash;
use crate::tr::model::extension::ModelExtensions;

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Plugin {
    // Откуда качаем
    #[serde(default)]
    source: Sources,
    // Версия
    #[serde(default)]
    version: Option<String>,
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

impl Plugin {
    pub fn source(&self) -> &Sources {
        &self.source
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn channel(&self) -> &Channels {
        &self.channel
    }

    pub fn freeze(&self) -> bool {
        self.freeze
    }

    pub fn force_update(&self) -> bool {
        self.force_update
    }
    /// Get link from models.
    pub async fn get_link<'a>(
        &'a self,
        name: &'a str,
        game_version: &'a str,
    ) -> Result<(String, ChooseHash, String)> {
        match self.source {
            Sources::Spigot => todo!(),
            Sources::Hangar => todo!(),
            Sources::Modrinth => ModrinthData::get_link(self, name, game_version).await,
            Sources::CurseForge => todo!(),
        }
    }
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
