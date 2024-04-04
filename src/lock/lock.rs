use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        core::{Core, Provider},
        Config,
    },
    errors::error::{ConfigErrors, LockErrors},
};

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    core: CoreMetaData,
    plugins: HashMap<String, ExtensionMetaData>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct CoreMetaData {
    name: Provider,
    version: Option<String>,
    build: Option<String>,
    path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
struct ExtensionMetaData {
    version: String,
    path: String,
}

impl Lock {
    /// Load lock from the specified file path
    pub async fn load(&mut self, path: &str) -> Result<(), ConfigErrors> {
        if !self.check_path(path).await {
            return Err(ConfigErrors::LoadCorrupt(format!(
                "No path like: {}, exist",
                path
            )));
        }
        info!("Starting to load lock from file: {}", path);

        // Read the file contents
        let toml = fs::read_to_string(&path).await?;
        info!("Lock file loaded successfully.");

        info!("Deserializing lock file contents...");
        // Deserialize the file contents into Lock struct
        let lock: Lock = toml::from_str(&toml)?;
        info!("Lock file deserialized successfully.");
        *self = lock;
        info!("Lock reload successfully");
        Ok(())
    }

    /// Create lock file
    pub async fn create(&self, path: &str) -> Result<(), ConfigErrors> {
        if !self.check_path(path).await {
            return Err(ConfigErrors::LoadCorrupt(format!(
                "No path like: {}, exist",
                path
            )));
        }
        info!("Creating lock file at path: {}", path);

        // Serialize the Lock struct to TOML
        let toml_content = toml::to_string_pretty(&self)?;

        // Open or create the lock file
        let mut file = File::create(&path).await?;
        info!("Lock file created.");

        // Write the serialized TOML content to the file
        file.write_all(toml_content.as_bytes()).await?;
        info!("Lock file content written successfully.");

        Ok(())
    }

    /// Re parsing file and load
    pub async fn reload(&mut self, path: &str) -> Result<(), ConfigErrors> {
        if !self.check_path(path).await {
            return Err(ConfigErrors::LoadCorrupt(format!(
                "No path like: {}, exist",
                path
            )));
        }
        info!("Reloading lock file from path: {}", path);
        // Read the file contents
        let toml_content = fs::read_to_string(&path).await?;
        info!("Lock file reloaded successfully.");

        info!("Deserializing lock file contents...");
        // Deserialize the file contents into Lock struct using serde_toml
        let lock: Lock = toml::from_str(&toml_content)?;
        info!("Lock file deserialized successfully.");
        *self = lock;
        info!("Lock reload successfully");
        Ok(())
    }

    /// Check if file exists in the system
    pub async fn check_path(&self, path: &str) -> bool {
        info!("Checking if lock file exists at path: {}", path);

        // Check if the lock file exists
        let exists = Path::new(&path).exists();
        info!("Lock file exists: {}", exists);

        exists
    }

    /// Save lock
    pub async fn save(&self, path: &str) -> Result<(), ConfigErrors> {
        if !self.check_path(path).await {
            return Err(ConfigErrors::LoadCorrupt(format!(
                "No path like: {}, exist",
                path
            )));
        }
        info!("Saving lock to file: {}", path);
        // Serialize the Lock struct to TOML
        let toml_content = toml::to_string_pretty(&self)?;

        // Open or create the lock file
        let mut file = File::create(&path).await?;
        info!("Lock file created.");

        // info!("Flush lock");
        // file.flush().await?;

        file.write_all(toml_content.as_bytes()).await?;
        info!("Lock file content written successfully.");

        Ok(())
    }

    ///Exist this item?
    pub async fn exist_plugin(&self, name: &str, version: &str) -> ExistState {
        if self.plugins.contains_key(name) {
            return ExistState::Exist;
        }
        if self.plugins.contains_key(name)
            && self
                .plugins
                .get(name)
                .map_or(false, |x| x.version == version)
        {
            return ExistState::DifferentVersion;
        }

        ExistState::None
    }

    pub async fn exist_core(&self, core: &Core, build: &str) -> ExistState {
        if self.core.name == core.provider
            && self.core.version == core.version
            && self.core.build == Some(build.to_owned())
        {
            return ExistState::Exist;
        }
        if self.core.version == core.version {
            return ExistState::DifferentVersion;
        }
        if self.core.build == core.build {
            return ExistState::DifferentBuild;
        }
        ExistState::None
    }

    /// Delete all core items from meta_data
    /// cause only one core can exist in one time
    pub async fn delete_core(&mut self) -> Result<(), LockErrors> {
        delete_file_by_path(&self.core.path).await
    }
    ///Delete plugin
    pub async fn delete_plugin(&mut self, name: &str) -> Result<(), LockErrors> {
        match self.plugins.remove(name) {
            Some(e) => delete_file_by_path(&e.path).await,
            None => Err(LockErrors::NotFound(name.to_string())),
        }
    }

    pub async fn remove_if_not_exist_plugin(&mut self, config: &Config) -> Result<(), LockErrors> {
        // Create list of keys to delete
        let mut keys_to_remove: Vec<String> = Vec::new();
        for i in self.plugins.keys() {
            if !config.plugins.contains_key(i) {
                keys_to_remove.push(i.to_owned());
            }
        }
        // Remove the plugins from self.plugins using the collected keys
        for i in keys_to_remove.iter() {
            let i = self.plugins.remove(i).unwrap();
            delete_file_by_path(&i.path).await?;
        }
        Ok(())
    }

    pub async fn remove_if_not_exist_core(&mut self, config: &Config) -> Result<(), LockErrors> {
        if self.core.name != config.core.provider {
            delete_file_by_path(&self.core.path).await?;
            self.core = CoreMetaData::default();
        }
        Ok(())
    }

    /// Converting [`Core`] to [`CoreMetaData`] and change values of core in [`Lock`]
    ///
    /// Need to clone [`Core`]
    pub async fn core_edit(&mut self, core: Core, path: String, build: String) {
        self.core = CoreMetaData {
            name: core.provider,
            version: core.version,
            build: Some(build),
            path,
        }
    }

    /// Push key: String, value: Version from plugin
    /// Path where plugin exist
    pub async fn plugin_add(&mut self, name: String, version: String, path_to_dir: &str) {
        let path = format!("{}/{}.jar", path_to_dir, &name);
        debug!("FN Plugin_add: path: {}", &path);
        let extension = ExtensionMetaData { version, path };
        debug!("FN Plugin_add: Extension: {:#?}", &extension);
        self.plugins.insert(name, extension);
        debug!("FN Plugin_add: self.plugins: {:#?}", &self.plugins);
    }
}

async fn delete_file(name: &str, download_dir: &str) -> Result<(), LockErrors> {
    let file_name = format!("{}.jar", name);
    let file_path = PathBuf::from(download_dir).join(file_name);
    debug!("file_path: {:#?}", file_path);
    fs::remove_file(&file_path).await.map_err(|e| e.into())
}

async fn delete_file_by_path(path: &str) -> Result<(), LockErrors> {
    if path.is_empty() {
        Ok(())
    } else {
        let path = OsStr::new(path);
        debug!("file_path: {:#?}", path);
        fs::remove_file(path).await.map_err(|e| e.into())
    }
}
pub enum ExistState {
    Exist,
    DifferentVersion,
    DifferentBuild,
    None,
}
