use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct ExtensionMeta {
    build: String,
    path: String,
}
impl ExtensionMeta {
    pub fn new(build: String, path: String) -> Self {
        Self { build, path }
    }

    pub fn build(&self) -> &str {
        self.build.as_ref()
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
