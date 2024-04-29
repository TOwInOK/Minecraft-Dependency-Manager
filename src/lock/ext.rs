use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionMeta {
    version: Option<String>,
    path: String,
}
impl ExtensionMeta {
    pub fn new(version: Option<String>, path: String) -> Self {
        Self { version, path }
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
