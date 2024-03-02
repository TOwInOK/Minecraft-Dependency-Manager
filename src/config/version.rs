use serde::{Serialize, Deserialize};

///Cores include version
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "core", content = "version")]
pub enum Versions {
    Purpur(String, bool),
    Paper(String, bool),
    Spigot(String, bool),
    Bucket(String, bool),
    Vanila(String, bool),
}

impl Default for Versions {
    fn default() -> Self {
        Versions::Vanila("1.20.4".to_string(), false)
    }
}