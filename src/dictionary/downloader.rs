use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct DownloaderMessages {
    calculate_hash: String,
    delete_exist_version: String,
    doest_need_to_update: String,
    download_file: String,
    file_downloaded: String,
    saving_file: String,
    write_to_lock: String,
    done: String,
}

impl DownloaderMessages {
    pub fn calculate_hash(&self) -> &str {
        &self.calculate_hash
    }

    pub fn delete_exist_version(&self) -> &str {
        &self.delete_exist_version
    }

    pub fn doest_need_to_update(&self) -> &str {
        &self.doest_need_to_update
    }

    pub fn download_file(&self) -> &str {
        &self.download_file
    }

    pub fn file_downloaded(&self) -> &str {
        &self.file_downloaded
    }

    pub fn saving_file(&self) -> &str {
        &self.saving_file
    }

    pub fn write_to_lock(&self) -> &str {
        &self.write_to_lock
    }

    pub fn done(&self) -> &str {
        &self.done
    }
}

impl Default for DownloaderMessages {
    fn default() -> Self {
        Self {
            calculate_hash: "Calculate hash...".into(),
            delete_exist_version: "Delete exist version!".into(),
            doest_need_to_update: "Does't need to update!".into(),
            download_file: "Download file...".into(),
            file_downloaded: "File downloaded!".into(),
            saving_file: "Saving file...".into(),
            write_to_lock: "Write to lock!".into(),
            done: "Done!".into(),
        }
    }
}
