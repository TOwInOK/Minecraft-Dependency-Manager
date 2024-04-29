use std::collections::HashMap;

use log::{debug, info};
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
        debug!("Plugins list: {:#?}", self.0);
        for (name, plugin) in self.0.iter() {
            // Получение ссылки, хэша и билда
            let (link, hash, build) = plugin.get_link(name, game_version).await?;
            //Проверка есть ли такой же Plugin в Lock
            if let Some(e) = lock.plugins().get(name) {
                match e.version() {
                    Some(e) => {
                        if *e == build && !plugin.force_update() || plugin.freeze() {
                            info!("Плагин {} не нуждается в обновлении", name);
                            return Ok(());
                        }
                    }
                    None => {}
                }
            }
            // Получение файла
            let file = plugin.get_file(name.to_string(), link, hash).await?;
            // Сохранение
            plugin.save_bytes(file, name).await?;
            // Добавление в Lock
            lock.plugins_mut().insert(name.to_string(), {
                ExtensionMeta::new(Some(build), format!("./plugins/{}.jar", name))
            });
            // Используем на каждой итерации, так как может возникнуть ошибка.
            lock.save().await?;
        }
        Ok(())
    }
}

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = "./plugins/";
}
