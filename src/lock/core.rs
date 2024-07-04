use serde::{Deserialize, Serialize};

use crate::{settings::core::Provider, tr::delete::Delete};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct CoreMeta {
    // Name of Core
    provider: Provider,
    // Version of core
    version: Option<String>,
    // Build version of Core
    build: Option<String>,
    // Path to core
    path: String,
}

impl CoreMeta {
    pub fn new(
        provider: Provider,
        version: Option<String>,
        build: Option<String>,
        path: String,
    ) -> Self {
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

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }
    pub async fn remove(&mut self) {
        self.delete(&self.path).await;
    }
}
impl Delete for CoreMeta {}
