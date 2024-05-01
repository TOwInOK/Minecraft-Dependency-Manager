use serde::{Deserialize, Serialize};

use crate::settings::core::{Core, Provider};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct CoreMeta {
    // Ядро
    provider: Provider,
    // Версия ядра
    version: String,
    // Версия билда ядра
    build: Option<String>,
    // Путь до ядра
    path: String,
}

impl CoreMeta {
    pub fn new(provider: Provider, version: String, build: Option<String>, path: String) -> Self {
        Self {
            provider,
            version,
            build,
            path,
        }
    }

    pub fn provider(&self) -> &Provider {
        &self.provider
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn build(&self) -> Option<&String> {
        self.build.as_ref()
    }
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
}
