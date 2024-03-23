use std::path::Path;

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{config::versions::Versions, errors::errors::{ConfigErrors, LockErrors}};


use tokio::{fs::{self, File}, io::AsyncWriteExt};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    pub meta_data: Vec<Meta>,
}

impl Lock {
    
    pub fn new() -> Self {
        Self { meta_data: Vec::<Meta>::new() }
    }
    
    /// Load lock from the specified file path
    pub async fn load() -> Result<Self, ConfigErrors> {
        let path = "./config.lock";
        info!("Starting to load lock from file: {}", path);

        // Read the file contents
        let toml = fs::read_to_string(&path).await?;
        info!("Lock file loaded successfully.");

        info!("Deserializing lock file contents...");
        // Deserialize the file contents into Lock struct
        let lock: Lock = toml::from_str(&toml)?;
        info!("Lock file deserialized successfully.");

        Ok(lock)
    }

    /// Create lock file
    pub async fn create(&self) -> Result<(), ConfigErrors> {
        let path = "./config.lock"; // Static path to the lock file
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

    /// Reparsing file and load
    pub async fn reload(self) -> Result<Self, ConfigErrors> {
        let path = "./config.lock"; // Static path to the lock file
        info!("Reloading lock file from path: {}", path);

        // Read the file contents
        let toml_content = fs::read_to_string(&path).await?;
        info!("Lock file reloaded successfully.");

        info!("Deserializing lock file contents...");
        // Deserialize the file contents into Lock struct using serde_toml
        let lock: Lock = toml::from_str(&toml_content)?;
        info!("Lock file deserialized successfully.");

        Ok(lock)
    }

    /// Check if file exists in the system
    pub async fn check(self) -> bool {
        let path = "./config.lock"; // Static path to the lock file
        info!("Checking if lock file exists at path: {}", path);

        // Check if the lock file exists
        let exists = Path::new(&path).exists();
        info!("Lock file exists: {}", exists);

        exists
    }

    /// Save lock
    pub async fn save(&self) -> Result<(), ConfigErrors> {
        let path = "./config.lock"; // Static path to the lock file
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

    /// Find item by name and version
    pub async fn exist(&self, meta: &Meta) -> ExistState {
        // Check if any MetaData in self.meta_data matches the provided meta
        if self.meta_data.iter().find(|&m| m == meta).is_some() {
            // MetaData with the same name and version exists
            ExistState::Exist
        } else {
            // Check if any MetaData in self.meta_data has the same name but different version
            if self.meta_data.iter().any(|m| m.get_name() == meta.get_name() && m.get_version() != meta.get_version()) {
                ExistState::DifferentVersion
            } else {
                ExistState::None
            }
        }
    }

    /// Delete all core items from meta_data
    pub async fn delete_core(&mut self, download_dir: &str) -> Result<(), LockErrors> {
        // Find the index of the Meta item with the specified name, if it exists
        match self.meta_data.iter().position(|item| {
            match item {
                Meta::Core(_) => true,
                _ => false,
            }
        }) {
            Some(index) => {
                let name = self.meta_data[index].get_name().to_owned(); // Get the name of the core
                self.meta_data.remove(index); // Remove the core from the meta_data vector
                delete_file(&name, download_dir).await // Delete the file associated with the core
            },
            None => Ok(())
        }
    }    
    ///Delete plugin
    pub async fn delete_plugin(&mut self, name: String, download_dir: &str)  {
        // Find the index of the Meta item with the specified name, if it exists
        if let Some(index) = self.meta_data.iter().position(|item| {
            match item  {
                Meta::Plugin(e) => e.name == name,
                _ => false,
            }
        }) {
            // Remove the item at the found index
            self.meta_data.remove(index);
        } else {
            // Handle the case when the item with the specified name is not found
            // You may want to add error handling or logging here
            // For now, let's just print a message
            println!("Item with name '{}' not found", name);
        }
    }
    ///Delete mod
    pub async fn delete_mod(&mut self, name: String, download_dir: &str)  {
        // Find the index of the Meta item with the specified name, if it exists
       if let Some(index) = self.meta_data.iter().position(|item| {
           match item  {
               Meta::Mod(e) => e.name == name,
               _ => false,
           }
       }) {
           // Remove the item at the found index
           self.meta_data.remove(index);
       } else {
           // Handle the case when the item with the specified name is not found
           // You may want to add error handling or logging here
           // For now, let's just print a message
           println!("Item with name '{}' not found", name);
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
            ExistState::None => todo!(),
        }
    }
}

async fn delete_file(name: &str, download_dir: &str) -> Result<(), LockErrors> {
    let mut name = name.to_string();
    name.push_str(".jar");
    let file_name = Path::new(&name);
    let file_path = Path::new(download_dir).join(file_name);
    fs::remove_file(file_path).await?;
    Ok(())
}
pub enum ExistState {
    Exist,
    DifferentVersion,
    None
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
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => &e.version,
        }
    }

    fn set_version(&mut self, ver: Versions) {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.version = ver,
        };
    }

    fn get_name(&self) -> &str {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => &e.name,
        }
    }

    fn set_name(&mut self, name: String) {
        match self {
            Meta::Core(e) | Meta::Plugin(e) | Meta::Mod(e) => e.name = name,
        };
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Eq)]
pub struct MetaData {
    pub name: String,
    pub version: Versions,
    // pub dependencies: Vec<String>,
}

impl MetaData {
    pub fn new(name: String, version: Versions) -> Self {
        Self { name, version }
    }
}
