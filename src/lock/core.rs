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
}

impl CoreMeta {
    pub fn new(provider: Provider, version: String, build: Option<String>) -> Self {
        Self {
            provider,
            version,
            build,
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
}

impl From<Core> for CoreMeta {
    fn from(value: Core) -> Self {
        Self {
            provider: value.provider().clone(),
            version: value.version().to_owned(),
            build: value.build().cloned(),
        }
    }
}
