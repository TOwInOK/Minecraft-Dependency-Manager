use serde::{Deserialize, Serialize};

use crate::errors::error::Result;
use crate::models::extensions::modrinth::ModrinthData;
use crate::tr::download::Download;
use crate::tr::hash::ChooseHash;
use crate::tr::model::extension::ModelExtensions;
use crate::tr::save::Save;

const PATH: &str = "./plugins/";

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
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

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
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
    pub async fn get_link(
        &self,
        name: &str,
        game_version: Option<&str>,
        loader: &str,
    ) -> Result<(String, ChooseHash, String)> {
        match self.source {
            Sources::Hangar => todo!(),
            Sources::Modrinth => ModrinthData::get_link(self, name, game_version, loader).await,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Sources {
    // Spigot, // bad api // deprecated
    Hangar, // ?
    #[default]
    Modrinth, // Favorite
            // CurseForge, // ? deprecated
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
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

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = PATH;
}
