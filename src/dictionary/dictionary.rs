use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::tr::load::Load;

use super::{
    config::ConfigMessages, defunct::DefunctMessages, downloader::DownloaderMessages,
    manager::ManagerMessages, model::ModelMessages,
};

#[derive(Deserialize, Serialize)]
pub struct MessageDictionary {
    intro: String,
    downloader: DownloaderMessages,
    manager: ManagerMessages,
    config: ConfigMessages,
    model: ModelMessages,
    defunct: DefunctMessages,
}

impl Default for MessageDictionary {
    fn default() -> Self {
        Self {
            intro: "MDM ready to work!".into(),
            downloader: DownloaderMessages::default(),
            manager: ManagerMessages::default(),
            config: ConfigMessages::default(),
            model: ModelMessages::default(),
            defunct: DefunctMessages::default(),
        }
    }
}
impl MessageDictionary {
    pub fn get_dict() -> Self {
        match MessageDictionary::load_sync() {
            Ok(e) => e,
            Err(e) => {
                error!("MessageDictionary: {}", e);
                warn!("Load default Dictionary");
                MessageDictionary::default()
            }
        }
    }

    pub fn intro(&self) -> &str {
        &self.intro
    }

    pub fn downloader(&self) -> &DownloaderMessages {
        &self.downloader
    }

    pub fn config(&self) -> &ConfigMessages {
        &self.config
    }
    pub fn manager(&self) -> &ManagerMessages {
        &self.manager
    }

    pub fn model(&self) -> &ModelMessages {
        &self.model
    }

    pub fn defunct(&self) -> &DefunctMessages {
        &self.defunct
    }
}

impl Load for MessageDictionary {
    const PATH: &'static str = "dictionary.toml";
}
