use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(untagged)]
pub enum Versions {
    #[serde(rename = "Version")]
    Version(String),
    #[default]
    Latest,
}
