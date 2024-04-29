use serde::{Deserialize, Serialize};

use crate::settings::extensions::plugin::Plugin;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtensionMeta {
    version: Option<String>,
    path: String,
}
impl ExtensionMeta {
    fn from_plugin(value: Plugin, path: &str) -> Self {
        Self {
            version: value.version().cloned(),
            path: path.to_owned(),
        }
    }
}
