mod core;
mod ext;

use serde::{Deserialize, Serialize};

use self::core::CoreMeta;
use self::ext::ExtensionMeta;
use crate::{
    settings::core::Core,
    tr::{load::Load, save::Save},
};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize)]
pub struct Lock {
    core: CoreMeta,
    plugins: HashMap<String, ExtensionMeta>,
    // mods: HashMap<String, ExtensionMeta>,
}
impl Lock {
    fn update_core(&mut self, core: Core) {
        self.core = core.into();
    }
}

impl Save for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Load for Lock {
    const PATH: &'static str = "./lock.toml";
}
