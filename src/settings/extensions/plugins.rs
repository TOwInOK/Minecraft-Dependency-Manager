use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tr::{download::Download, save::Save};

use super::plugin::{Channels, Plugin};

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Plugins(HashMap<String, Plugin>);


impl Plugins {
    pub fn new(items: HashMap<String, Plugin>) -> Self {
        Self(items)
    }

    pub fn items(&self) -> &HashMap<String, Plugin> {
        &self.0
    }
}

impl Channels {
    pub async fn get_str(&self) -> &'static str {
        match self {
            Channels::Release => "release",
            Channels::Beta => "beta",
            Channels::Alpha => "alpha",
        }
    }
}

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = "./mods/";
}
