use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Versions {
    #[serde(rename = "Version")]
    Version(String),
    Latest,
}

impl Default for Versions {
    fn default() -> Self {
        Versions::Latest
    }
}
