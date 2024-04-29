pub mod core;
pub mod extensions;
pub mod additions;

use serde::{Deserialize, Serialize};

use crate::tr::{load::Load, save::Save};

use self::{
    additions::Additions, core::Core, extensions::{mods::Mods, plugins::Plugins}
};
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Settings {
    #[serde(default)]
    core: Core,
    #[serde(default)]
    mods: Option<Mods>,
    #[serde(default)]
    plugins: Option<Plugins>,
    #[serde(default)]
    addtions: Option<Additions>,
}

impl Settings {
    pub fn core(&self) -> &Core {
        &self.core
    }

    pub fn mods(&self) -> Option<&Mods> {
        self.mods.as_ref()
    }

    pub fn plugins(&self) -> Option<&Plugins> {
        self.plugins.as_ref()
    }
    
    pub fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }
    
    pub fn mods_mut(&mut self) -> &mut Option<Mods> {
        &mut self.mods
    }
    
    pub fn plugins_mut(&mut self) -> &mut Option<Plugins> {
        &mut self.plugins
    }
    
    pub fn set_core(&mut self, core: Core) {
        self.core = core;
    }
    
    pub fn set_mods(&mut self, mods: Option<Mods>) {
        self.mods = mods;
    }
    
    pub fn set_plugins(&mut self, plugins: Option<Plugins>) {
        self.plugins = plugins;
    }
}

impl Save for Settings {
    const PATH: &'static str = "./settings.toml";
}
impl Load for Settings {
    const PATH: &'static str = "./settings.toml";
}
