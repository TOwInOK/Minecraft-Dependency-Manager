use std::sync::Arc;

use indicatif::MultiProgress;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::errors::error::Result;
use crate::lock::Lock;
use crate::models::cores::folia::Folia;
use crate::models::cores::paper::Paper;
use crate::models::cores::purpur::Purpur;
use crate::models::cores::vanilla::Vanilla;
use crate::models::cores::velocity::Velocity;
use crate::models::cores::waterfall::Waterfall;
use crate::tr::hash::ChooseHash;
use crate::tr::model::core::ModelCore;
use crate::tr::{download::Download, save::Save};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Core {
    // Ядро
    #[serde(default)]
    provider: Provider,
    // Версия ядра
    #[serde(default = "version")]
    version: String,
    // Версия билда ядра
    #[serde(default)]
    build: Option<String>,
    // Приостановить обновление
    #[serde(default)]
    freeze: bool,
    // Нужно обновить
    #[serde(default)]
    force_update: bool,
}

fn version() -> String {
    // warn!("We use default core path!");
    "Latest".to_string()
}

impl Core {
    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    pub fn build(&self) -> Option<&String> {
        self.build.as_ref()
    }

    pub fn freeze(&self) -> bool {
        self.freeze
    }

    pub fn force_update(&self) -> bool {
        self.force_update
    }

    pub fn set_provider(&mut self, provider: Provider) {
        self.provider = provider;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn set_build(&mut self, build: Option<String>) {
        self.build = build;
    }

    pub fn set_freeze(&mut self, freeze: bool) {
        self.freeze = freeze;
    }

    pub fn set_force_update(&mut self, force_update: bool) {
        self.force_update = force_update;
    }
    /// Скачиваем `Core` и сохраняем его по стандартному пути.
    pub async fn download(
        &self,
        lock: &Arc<Mutex<Lock>>,
        mpb: &Arc<Mutex<MultiProgress>>,
    ) -> Result<()> {
        let (link, hash, build) = self.get_link().await?;
        let mut lock = lock.lock().await;
        if let Some(e) = lock.core().build() {
            debug!("lock build: {} / build: {}", &e, &build);
            if *e == build && !self.force_update || self.freeze {
                info!("Ядро не нуждается в обновлении");
                return Ok(());
            }
        }
        let mpb = mpb.lock().await;
        let file = Core::get_file(self.provider.as_str().to_string(), link, hash, &mpb).await?;
        Core::save_bytes(file, self.provider().as_str()).await?;
        *lock.core_mut() = self.clone().into();
        lock.save().await
    }
    async fn get_link(&self) -> Result<(String, ChooseHash, String)> {
        match self.provider {
            Provider::Vanilla => Vanilla::get_link(self).await,
            Provider::Paper => Paper::get_link(self).await,
            Provider::Folia => Folia::get_link(self).await,
            Provider::Purpur => Purpur::get_link(self).await,
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
            Provider::Waterfall => Waterfall::get_link(self).await,
            Provider::Velocity => Velocity::get_link(self).await,
        }
    }
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
    pub fn as_str(&self) -> &'static str {
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

impl Download for Core {}
impl Save for Core {
    const PATH: &'static str = "./";
}
