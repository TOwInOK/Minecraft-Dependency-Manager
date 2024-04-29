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
    plugins: HashMap<String, ExtensionMeta>,
    mods: HashMap<String, ExtensionMeta>,
}
impl Lock {
    pub fn core(&self) -> &CoreMeta {
        &self.core
    }

    pub fn plugins(&self) -> &HashMap<String, ExtensionMeta> {
        &self.plugins
    }

    pub fn core_mut(&mut self) -> &mut CoreMeta {
        &mut self.core
    }

    pub fn plugins_mut(&mut self) -> &mut HashMap<String, ExtensionMeta> {
        &mut self.plugins
    }
}

impl Save for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Load for Lock {
    const PATH: &'static str = "./lock.toml";
}
