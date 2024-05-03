use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Additions {
    // git link
    #[serde(default)]
    source: Option<String>,
    // git key
    #[serde(default)]
    key: Option<String>,
}

impl Additions {
    pub fn new(source: Option<String>, key: Option<String>) -> Self {
        Self { source, key }
    }
}
