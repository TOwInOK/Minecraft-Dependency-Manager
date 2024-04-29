use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::error::Result;
use crate::lock::ext::ExtensionMeta;
use crate::lock::Lock;
use crate::tr::{download::Download, save::Save};

use super::plugin::Plugin;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Plugins(HashMap<String, Plugin>);

impl Plugins {
    pub fn new(items: HashMap<String, Plugin>) -> Self {
        Self(items)
    }

    pub fn items(&self) -> &HashMap<String, Plugin> {
        &self.0
    }

    pub async fn download_all(&self, game_version: &str, lock: &mut Lock) -> Result<()> {
        for (name, plugin) in self.0.iter() {
            // Получение ссылки, хэша и билда
            let (link, hash, build) = plugin.get_link(name, game_version).await?;
            //Проверка есть ли такой же Plugin в Lock
            if let Some(e) = lock.plugins().get(name) {
                match e.version() {
                    Some(e) => {
                        if *e == build {
                            return Ok(());
                        }
                    }
                    None => continue,
                }
            }
            // Получение файла
            let file = plugin.get_file(link, hash).await?;
            // Сохранение
            plugin.save_bytes(file, name).await?;
            // Добавление в Lock
            lock.plugins_mut().insert(name.to_string(), {
                ExtensionMeta::new(Some(build), format!("./plugins/{}.jar", name))
            });
            lock.save().await?;
        }
        Ok(())
    }
}

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = "./plugins/";
}
