use std::path::{Path, PathBuf};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{
    config::versions::Versions,
    errors::error::{ConfigErrors, LockErrors},
};

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    pub meta_data: Vec<Meta>,
}

impl Lock {
    pub fn new() -> Self {
        Self {
            meta_data: Vec::<Meta>::new(),
        }
    }

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
        // info!("{:#?}", self.meta_data);

        file.write_all(toml_content.as_bytes()).await?;
        info!("Lock file content written successfully.");

        Ok(())
    }

    ///Exist this item?
    pub async fn exist(&self, outer_meta: &Meta) -> ExistState {
        if self.meta_data.contains(outer_meta) {
            return ExistState::Exist;
        }
        if self.find_different_version(outer_meta).is_some() {
            return ExistState::DifferentVersion;
        }
        if self.find_different_build(outer_meta).is_some() {
            return ExistState::DifferentBuild;
        }
        ExistState::None
    }

    fn find_different_version(&self, outer_meta: &Meta) -> Option<&Meta> {
        self.meta_data.iter().find(|m| {
            m.get_name() == outer_meta.get_name() && m.get_version() != outer_meta.get_version()
        })
    }

    fn find_different_build(&self, outer_meta: &Meta) -> Option<&Meta> {
        self.meta_data.iter().find(|m| {
            m.get_name() == outer_meta.get_name()
                && m.get_version() == outer_meta.get_version()
                && m.get_dependencies() != outer_meta.get_dependencies()
        })
    }

    /// Delete all core items from meta_data
    /// cause only one core can exist in one time
    pub async fn delete_core(&mut self, download_dir: &str) -> Result<(), LockErrors> {
        match self
            .meta_data
            .iter()
            .position(|item| matches!(item, Meta::Core(_)))
        {
            Some(index) => {
                let core_name = self.meta_data[index].get_name().to_owned();
                debug!("Core_name: {}", core_name);

                self.meta_data.remove(index); // Удаление ядра из вектора meta_data

                // Удаление файла, связанного с ядром
                delete_file(&core_name, download_dir).await
            }
            None => Ok(()),
        }
    }
    ///Delete plugin
    pub async fn delete_plugin(
        &mut self,
        name: String,
        download_dir: &str,
    ) -> Result<(), LockErrors> {
        match self
            .meta_data
            .iter()
            .position(|item| matches!(item, Meta::Plugin(e) if e.name == name))
        {
            Some(index) => {
                self.meta_data.remove(index);
                delete_file(&name, download_dir).await
            }
            None => Ok(()),
        }
    }

    ///Delete mod
    pub async fn _delete_mod(
        &mut self,
        name: String,
        download_dir: &str,
    ) -> Result<(), LockErrors> {
        match self
            .meta_data
            .iter()
            .position(|item| matches!(item, Meta::Mod(e) if e.name == name))
        {
            Some(index) => {
                self.meta_data.remove(index);
                delete_file(&name, download_dir).await
            }
            None => Ok(()),
        }
    }

    ///add item
    pub async fn add(&mut self, meta: Meta) {
        self.meta_data.push(meta)
    }
    /// Update item
    pub async fn update(&mut self, updated_meta: Meta) {
        match self.exist(&updated_meta).await {
            ExistState::Exist => todo!(),
            ExistState::DifferentVersion => todo!(),
            ExistState::DifferentBuild => todo!(),
            ExistState::None => todo!(),
        }
    }
}

async fn delete_file(name: &str, download_dir: &str) -> Result<(), LockErrors> {
    let file_name = format!("{}.jar", name);
    let file_path = PathBuf::from(download_dir).join(file_name);
    debug!("file_path: {:#?}", file_path);
    fs::remove_file(&file_path).await.map_err(|e| e.into())
}
pub enum ExistState {
    Exist,
    DifferentVersion,
    DifferentBuild,
    None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub enum Meta {
    Core(MetaData),
    Plugin(MetaData),
    Mod(MetaData),
}
impl Meta {
    fn get_version(&self) -> &Versions {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.get_version(),
        }
    }

    fn set_version(&mut self, version: Versions) {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.set_version(version),
        };
    }

    fn get_name(&self) -> &str {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.get_name(),
        }
    }

    fn set_name(&mut self, name: String) {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.set_name(name),
        };
    }
    pub fn get_build(&self) -> Option<&str> {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.get_build(),
        }
    }

    pub fn set_build(&mut self, build: Option<String>) {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.set_build(build),
        }
    }
    pub fn get_dependencies(&self) -> Option<&Vec<String>> {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.get_dependencies(),
        }
    }
    pub fn set_dependencies(&mut self, deps: Option<Vec<String>>) {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.set_dependencies(deps),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct MetaData {
    pub name: String,
    pub version: Versions,
    pub build: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

impl MetaData {
    pub fn new(
        name: String,
        version: Versions,
        build: Option<String>,
        dependencies: Option<Vec<String>>,
    ) -> Self {
        Self {
            name,
            version,
            build,
            dependencies,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_version(&self) -> &Versions {
        &self.version
    }

    pub fn set_version(&mut self, version: Versions) {
        self.version = version;
    }

    pub fn get_build(&self) -> Option<&str> {
        self.build.as_deref()
    }

    pub fn set_build(&mut self, build: Option<String>) {
        self.build = build;
    }
    pub fn get_dependencies(&self) -> Option<&Vec<String>> {
        self.dependencies.as_ref()
    }
    pub fn set_dependencies(&mut self, deps: Option<Vec<String>>) {
        self.dependencies = deps;
    }
}
