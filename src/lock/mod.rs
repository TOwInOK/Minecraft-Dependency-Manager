pub mod core;
pub mod ext;

use serde::{Deserialize, Serialize};

use self::core::CoreMeta;
use self::ext::ExtensionMeta;
use crate::errors::error::Result;
use crate::settings::Settings;
use crate::{
    settings::core::Core,
    tr::{load::Load, save::Save},
};
use std::{collections::HashMap, fs};

#[derive(Default, Serialize, Deserialize)]
pub struct Lock {
    core: CoreMeta,
    plugins: ExtensionMetaList,
    mods: ExtensionMetaList,
}
impl Lock {
    pub fn core(&self) -> &CoreMeta {
        &self.core
    }
    pub fn plugins(&self) -> &ExtensionMetaList {
        &self.plugins
    }
    pub fn core_mut(&mut self) -> &mut CoreMeta {
        &mut self.core
    }
    pub fn plugins_mut(&mut self) -> &mut ExtensionMetaList {
        &mut self.plugins
    }
    pub fn set_core(&mut self, value: Core) {
        self.core = value.into();
    }
    pub fn remove_plugin(&mut self, key: &str) -> Result<()> {
        self.plugins.remove(key)
    }
    pub fn remove_mod(&mut self, key: &str) -> Result<()> {
        self.mods.remove(key)
    }
    //delete and make the current core the default
    pub fn remove_core(&mut self) -> Result<()> {
        fs::remove_file(self.core().path())?;
        self.core = CoreMeta::default();
        Ok(())
    }
    pub fn remove_nonexistent(&mut self, settings: &Settings) -> Result<()> {
        let plugin_keys: Vec<String> = self.plugins().0.keys().cloned().collect();
        // TODO: let mods_keys
        if let Some(e) = settings.plugins() {
            for key in plugin_keys {
                if !e.items().contains_key(&key) {
                    self.remove_plugin(key.as_str())?;
                }
            }
        }
        // TODO: add remover for mods
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ExtensionMetaList(HashMap<String, ExtensionMeta>);
impl ExtensionMetaList {
    pub fn get(&self, key: &str) -> Option<&ExtensionMeta> {
        self.0.get(key)
    }
    pub fn insert(&mut self, key: String, value: ExtensionMeta) {
        self.0.insert(key, value);
    }
    pub fn remove(&mut self, key: &str) -> Result<()> {
        let value = self.0.remove(key);
        match value {
            Some(e) => Ok(fs::remove_file(e.path())?),
            None => Ok(()),
        }
    }
    pub fn update(&mut self, key: String, value: ExtensionMeta) -> Result<()> {
        self.remove(&key)?;
        self.insert(key, value);
        Ok(())
    }
}

impl Save for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Load for Lock {
    const PATH: &'static str = "./lock.toml";
}
