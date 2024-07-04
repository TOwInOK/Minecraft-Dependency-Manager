mod config;
mod defunct;
mod downloader;
mod manager;
mod model;

use std::{fs::File, io::Write};

use config::ConfigMessages;
use defunct::DefunctMessages;
use downloader::DownloaderMessages;
use log::{error, warn};
use manager::ManagerMessages;
use model::ModelMessages;
use serde::{Deserialize, Serialize};

use crate::tr::load::Load;

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
    pub fn get_dict() -> crate::errors::error::Result<Self> {
        '_default_language_scope: {
            let path = <MessageDictionary as Load>::PATH;
            if File::open(path).is_err() {
                let default = MessageDictionary::default();
                warn!("Create default language file");
                let mut file = File::create(path)?;
                let toml_default = toml::to_string_pretty(&default)?;
                file.write_all(toml_default.as_bytes())?;
            }
        }
        match MessageDictionary::load_sync() {
            Ok(e) => Ok(e),
            Err(e) => {
                error!("MessageDictionary: {}", e);
                warn!("Load default Dictionary");
                Ok(MessageDictionary::default())
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
