use serde::{Deserialize, Serialize};

///Cores include version
///# Explanetion
///
///#### Fist string is `version`
///version can be: `latest` and string-numbers like `"1.14.4"`
///#### Second value is `freez`
///when freez is true we don't update version like `"1.1.1" (#fdf2134)` to `"1.1.1" (#fdf2154)`
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "core", content = "version")]
pub enum Versions {
    Purpur(String, bool),
    Paper(String, bool),
    Spigot(String, bool),
    Bucket(String, bool),
    Vanilla(String, bool),
}

impl Default for Versions {
    fn default() -> Self {
        Versions::Vanilla("latest".to_string(), false)
    }
}
