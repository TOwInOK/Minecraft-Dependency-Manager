use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::{
    errors::error::Result,
    lock::core::CoreMeta,
    models::cores::{
        folia::Folia, paper::Paper, purpur::Purpur, vanilla::Vanilla, velocity::Velocity,
        waterfall::Waterfall,
    },
    pb,
    tr::{download::Download, hash::ChooseHash, model::core::ModelCore, save::Save},
    DICTIONARY, LOCK, MPB,
};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Core {
    // Core
    #[serde(default)]
    provider: Provider,
    // Version of Core
    version: Option<String>,
    // Build version of Core
    #[serde(default)]
    build: Option<String>,
    // Freeze updates?
    #[serde(default)]
    freeze: bool,
    // Force update it?
    #[serde(default)]
    force_update: bool,
}

impl Core {
    pub fn to_meta(self, build: String) -> CoreMeta {
        let path = format!("./{}.jar", &self.provider.as_str().to_string());
        CoreMeta::new(self.provider, self.version, Some(build), path)
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
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
        self.version = Some(version);
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
    /// Download `Core` and save it to standard path.
    pub async fn download(&self) -> Result<()> {
        let pb = pb!(MPB, self.provider.as_str());

        let (link, hash, build) = self.get_link(&pb).await?;
        trace!("link: {}, hash: {}", &link, &hash);
        if let Some(e) = LOCK.lock().await.core().build() {
            trace!("lock build: {} / build: {}", &e, &build);
            if *e == build && (!self.force_update || self.freeze) {
                pb.set_message(DICTIONARY.downloader().doest_need_to_update());
                sleep(Duration::from_secs(1)).await;
                pb.finish_and_clear();
                return Ok(());
            }
        }
        pb.set_message(DICTIONARY.downloader().download_file());
        let file = Core::get_file(link, hash, &pb).await?;
        debug!("file: => {} | got", self.provider().as_str());

        pb.set_message(DICTIONARY.downloader().delete_exist_version());
        {
            LOCK.lock().await.remove_core().await;
        }

        debug!("file: => {} | saving", self.provider().as_str());
        pb.set_message(DICTIONARY.downloader().saving_file());
        Core::save_bytes(file, self.provider().as_str()).await?;
        debug!("file: => {} | saved", self.provider().as_str());

        debug!("Data: => {} | start locking", self.provider().as_str());

        pb.set_message(DICTIONARY.downloader().write_to_lock());
        {
            let mut lock = LOCK.lock().await;
            *lock.core_mut() = self.clone().to_meta(build);
            lock.save().await?;
        }

        debug!(
            "Data: => {} | written into lock file",
            self.provider().as_str()
        );

        pb.set_message(DICTIONARY.downloader().done());
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
