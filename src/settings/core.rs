use crate::tr::load::Load;
use std::{sync::Arc, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, time::sleep};

use crate::{
    dictionary::pb_messages::PbMessages,
    errors::error::Result,
    lock::{core::CoreMeta, Lock},
    models::cores::{
        folia::Folia, paper::Paper, purpur::Purpur, vanilla::Vanilla, velocity::Velocity,
        waterfall::Waterfall,
    },
    pb,
    tr::{
        delete::Delete, download::Download, hash::ChooseHash, model::core::ModelCore, save::Save,
    },
};

lazy_static! {
    static ref DICT: PbMessages = PbMessages::load_sync().unwrap();
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Core {
    // Ядро
    #[serde(default)]
    provider: Provider,
    // Версия ядра
    // Change to Enum!
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
    pub fn to_meta(self, build: String) -> CoreMeta {
        let path = format!("./{}.jar", &self.provider.as_str().to_string());
        CoreMeta::new(self.provider, self.version, Some(build), path)
    }

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
    pub async fn download(&self, lock: Arc<Mutex<Lock>>, mpb: Arc<MultiProgress>) -> Result<()> {
        let pb = pb!(mpb, self.provider.as_str());

        let (link, hash, build) = self.get_link(&pb).await?;
        trace!("link: {}, hash: {}", &link, &hash);
        if let Some(e) = lock.lock().await.core().build() {
            trace!("lock build: {} / build: {}", &e, &build);
            if *e == build && (!self.force_update || self.freeze) {
                pb.set_message(&DICT.doest_need_to_update);
                sleep(Duration::from_secs(1)).await;
                pb.finish_and_clear();
                return Ok(());
            }
        }
        pb.set_message(&DICT.download_file);
        let file = Core::get_file(link, hash, &pb).await?;
        debug!("file: => {} | got", self.provider().as_str());
        debug!("file: => {} | saving", self.provider().as_str());

        pb.set_message(&DICT.delete_exist_version);
        {
            self.delete(lock.lock().await.core().path()).await;
        }

        pb.set_message(&DICT.saving_file);
        Core::save_bytes(file, self.provider().as_str()).await?;

        debug!("file: => {} | saved", self.provider().as_str());
        debug!("Data: => {} | start locking", self.provider().as_str());

        pb.set_message(&DICT.write_to_lock);
        {
            let mut lock = lock.lock().await;
            *lock.core_mut() = self.clone().to_meta(build);
            lock.save().await?;
        }

        debug!(
            "Data: => {} | written into lock file",
            self.provider().as_str()
        );

        pb.set_message(&DICT.done);
        pb.finish_and_clear();
        Ok(())
    }
    async fn get_link(&self, pb: &ProgressBar) -> Result<(String, ChooseHash, String)> {
        match self.provider {
            Provider::Vanilla => Vanilla::get_link(self, pb).await,
            Provider::Paper => Paper::get_link(self, pb).await,
            Provider::Folia => Folia::get_link(self, pb).await,
            Provider::Purpur => Purpur::get_link(self, pb).await,
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
            Provider::Waterfall => Waterfall::get_link(self, pb).await,
            Provider::Velocity => Velocity::get_link(self, pb).await,
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
    Purpur, // done, good api
    Fabric, // in work, api with out hash
    //https://meta.fabricmc.net/v2/versions/game <- version check /v2/versions/intermediary give only stable
    // or https://meta.fabricmc.net/v1/versions/game/1.14.4. If version is empty, response is empty.
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
impl Delete for Core {}
