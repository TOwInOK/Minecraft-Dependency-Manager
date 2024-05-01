pub mod core;
pub mod ext;

use serde::{Deserialize, Serialize};

use self::core::CoreMeta;
use self::ext::ExtensionMeta;
use crate::tr::{load::Load, save::Save};
use std::collections::HashMap;

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
}

#[derive(Default, Serialize, Deserialize)]
pub struct ExtensionMetaList(HashMap<String, ExtensionMeta>);
impl ExtensionMetaList {
    pub fn get(&self, key: &str) -> Option<&ExtensionMeta> {
        self.0.get(key)
    }
    pub fn set(&mut self, key: String, value: ExtensionMeta) {
        self.0.insert(key, value);
    }
}

impl Save for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Load for Lock {
    const PATH: &'static str = "./lock.toml";
}
