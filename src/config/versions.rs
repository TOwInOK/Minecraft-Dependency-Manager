use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[derive(Debug, Default, PartialEq, Clone, Eq)]
pub enum Versions {
    Version(String),
    #[default]
    Latest,
}

impl Versions {
    pub async fn is_latest(&self) -> bool {
        match self {
            Versions::Version(_) => false,
            Versions::Latest => true,
        }
    }
    pub async fn get_version(&self) -> &str {
        match self {
            Versions::Version(e) => e.deref(),
            Versions::Latest => "latest",
        }
    }
}

impl<'de> Deserialize<'de> for Versions {
    fn deserialize<D>(deserializer: D) -> Result<Versions, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        if s.to_lowercase() == "latest" || s.is_empty() {
            Ok(Versions::Latest)
        } else {
            Ok(Versions::Version(s))
        }
    }
}

impl Serialize for Versions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Versions::Version(s) => serializer.serialize_str(s),
            Versions::Latest => serializer.serialize_str("latest"),
        }
    }
}
