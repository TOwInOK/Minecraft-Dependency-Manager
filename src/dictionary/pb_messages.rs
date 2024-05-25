use serde::{Deserialize, Serialize};

use crate::tr::load::Load;

#[derive(Deserialize, Serialize)]
pub struct PbMessages {
    pub calculate_hash: String,
    pub delete_exist_version: String,
    pub doest_need_to_update: String,
    pub done: String,
    pub download_file: String,
    pub file_downloaded: String,
    pub restart: String,
    pub saving_file: String,
    pub start_new_iteration: String,
    pub stop_iteration: String,
    pub waiting_new_iteration: String,
    pub write_to_lock: String,
    // Config
    pub find_changes_in_settings: String,
    pub settings_changed: String,
    pub settings_has_rewrited: String,
    pub settings_same: String,
    // Intro
    pub intro: String,
    // Models messages
    pub init_work: String,
    pub finding_version: String,
    pub make_link: String,
    // Remove nonexist
    pub start_remove_nonexist: String,
    pub remove_nonexist: String,
}

impl Default for PbMessages {
    fn default() -> Self {
        Self {
            restart: "Restart...".into(),
            stop_iteration: "Stop iteration!".into(),
            start_new_iteration: "Start new iteration!".into(),
            waiting_new_iteration: "Waiting new iteration...".into(),
            doest_need_to_update: "Does't need to update!".into(),
            delete_exist_version: "Delete exist version!".into(),
            download_file: "Download file...".into(),
            saving_file: "Saving file...".into(),
            write_to_lock: "Write to lock!".into(),
            done: "Done!".into(),
            file_downloaded: "File downloaded!".into(),
            calculate_hash: "Calculate hash...".into(),
            find_changes_in_settings: "Find some changes in settings!".into(),
            settings_changed: "Settings changed".into(),
            settings_has_rewrited: "Settings has rewrited".into(),
            settings_same: "Settings same".into(),
            intro: "MAC ready to work!".into(),
            init_work: "Generating link...".into(),
            finding_version: "Finding version...".into(),
            make_link: "Make link...".into(),
            start_remove_nonexist: "Start remove nonexist".into(),
            remove_nonexist: "Remove nonexist".into(),
        }
    }
}
impl PbMessages {
    pub fn remove_if_nonexist(&self, name: &str) -> String {
        format!("{}: {}", self.remove_nonexist, name)
    }
}

impl Load for PbMessages {
    const PATH: &'static str = "language.toml";
}
