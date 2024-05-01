use std::collections::HashMap;
use std::sync::Arc;

use futures_util::future::join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::errors::error::Result;
use crate::lock::ext::ExtensionMeta;
use crate::lock::Lock;
use crate::tr::{download::Download, save::Save};

use super::plugin::Plugin;

const PATH: &'static str = "./plugins/";

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Plugins(HashMap<String, Plugin>);

impl Plugins {
    pub fn new(items: HashMap<String, Plugin>) -> Self {
        Self(items)
    }

    pub fn items(&self) -> &HashMap<String, Plugin> {
        &self.0
    }

    pub async fn download_all(
        &self,
        game_version: &str,
        lock: &Arc<Mutex<Lock>>,
        mpb: &Arc<Mutex<MultiProgress>>,
    ) -> Result<()> {
        let mut link_list = Vec::new();
        let mut handler_list: Vec<
            tokio::task::JoinHandle<std::prelude::v1::Result<(), crate::errors::error::Error>>,
        > = Vec::new();
        // Make link_list
        // Check plugins in List
        for (name, plugin) in self.0.clone() {
            // Get link
            let (link, hash, build) = plugin.get_link(&name, game_version).await?;

            // Init name for PB
            let cloned_name = name.clone();
            let name_closure = move |_: &ProgressState, f: &mut dyn std::fmt::Write| {
                f.write_str(&cloned_name.clone()).unwrap();
            };
            // PB init
            let pb = mpb.lock().await.add(ProgressBar::new_spinner());
            // PB style
            pb.set_style(
                ProgressStyle::with_template("Package:: {name:.blue} >>> {msg:.blue}")
                    .unwrap()
                    .with_key("name", name_closure),
            );
            // Check meta
            if let Some(plugin_meta) = lock.lock().await.plugins().get(&name) {
                let local_build = plugin_meta.build();
                // Need to download?
                if *local_build == build && !plugin.force_update() || plugin.freeze() {
                    pb.finish_with_message("Does't need to update");
                    continue;
                }
            }
            link_list.push((link, hash, build, name.to_owned(), pb))
        }
        // Make handler_list
        for (link, hash, build, name, pb) in link_list {
            let lock = Arc::clone(lock);

            handler_list.push(tokio::spawn(async move {
                // get lock
                let mut lock = lock.lock().await;
                // get file
                let file = Plugin::get_file(name.to_owned(), link, hash, &pb).await?;
                pb.set_message("Remove exist version");
                //delete prevision item
                lock.remove_plugin(&name)?;
                pb.set_message("Saving...");
                // save on disk
                Plugin::save_bytes(file, name.as_str()).await?;
                pb.set_message("Logging...");
                //save in lock
                lock.plugins_mut().insert(name.to_string(), {
                    ExtensionMeta::new(build, format!("{}{}.jar", PATH, name))
                });
                let res = lock.save().await?;
                pb.finish_with_message("Done");
                Ok(res)
            }));
        }
        join_all(handler_list).await;
        Ok(())
    }
}

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = PATH;
}
