use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct DefunctMessages {
    start_remove_defunct: String,
    remove_defunct: String,
}

impl DefunctMessages {
    pub fn start_remove_defunct(&self) -> &str {
        &self.start_remove_defunct
    }

    pub fn remove_defunct(&self) -> &str {
        &self.remove_defunct
    }
    pub fn remove_if_defunct(&self, name: &str) -> String {
        format!("{}: {}", self.remove_defunct(), name)
    }
}

impl Default for DefunctMessages {
    fn default() -> Self {
        Self {
            start_remove_defunct: "Start remove defunct".into(),
            remove_defunct: "Remove defunct".into(),
        }
    }
}
