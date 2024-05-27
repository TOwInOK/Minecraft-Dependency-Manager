use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ConfigMessages {
    find_changes_in_settings: String,
    settings_changed: String,
    settings_rewritten: String,
    settings_same: String,
}

impl ConfigMessages {
    pub fn find_changes_in_settings(&self) -> &str {
        &self.find_changes_in_settings
    }

    pub fn settings_changed(&self) -> &str {
        &self.settings_changed
    }

    pub fn settings_rewritten(&self) -> &str {
        &self.settings_rewritten
    }

    pub fn settings_same(&self) -> &str {
        &self.settings_same
    }
}
impl Default for ConfigMessages {
    fn default() -> Self {
        Self {
            find_changes_in_settings: "Find some changes in settings!".into(),
            settings_changed: "Settings changed".into(),
            settings_rewritten: "Settings rewritten".into(),
            settings_same: "Settings same".into(),
        }
    }
}
