use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Additions {
    // git link
    #[serde(default)]
    source: Option<String>,
    // git key
    #[serde(default)]
    key: Option<String>,
    // duration of delay between download intervals
    #[serde(default = "duration_default")]
    duration: Option<u64>,
}

fn duration_default() -> Option<u64> {
    Some(300)
}

impl Additions {
    pub fn new(source: Option<String>, key: Option<String>, duration: Option<u64>) -> Self {
        Self {
            source,
            key,
            duration,
        }
    }

    pub fn source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    pub fn key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    pub fn duration(&self) -> Option<u64> {
        self.duration
    }
}
