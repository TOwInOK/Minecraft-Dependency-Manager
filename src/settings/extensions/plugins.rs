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
        //make list of links
        let mut link_list = Vec::new();
        let mut handler_list = Vec::new();
        for (name, plugin) in self.0.clone() {
            let (link, hash, build) = plugin.get_link(&name, game_version).await?;
            if let Some(plugin_meta) = lock.lock().await.plugins().get(&name) {
                let local_build = plugin_meta.build();
                if *local_build == build && !plugin.force_update() || plugin.freeze() {
                    // PB style, init
                    let name_closure = move |_: &ProgressState, f: &mut dyn std::fmt::Write| {
                        f.write_str(&name).unwrap();
                    };
                    let pb = mpb.lock().await.add(ProgressBar::new_spinner());
                    pb.set_style(
                        ProgressStyle::with_template("Package:: {name:.blue} >>> {msg:.blue}")
                            .unwrap()
                            .with_key("name", name_closure),
                    );
                    pb.finish_with_message("Does't need to update");
                    continue;
                }
            }
            link_list.push((link, hash, build, name.to_owned()))
        }
        for (link, hash, build, name) in link_list {
            let lock = Arc::clone(lock);
            let mpb = Arc::clone(mpb);
            handler_list.push(tokio::spawn(async move {
                let mpb = mpb.lock().await;
                let file = Plugin::get_file(name.to_owned(), link, hash, &mpb).await?;
                let mut lock = lock.lock().await;
                Plugin::save_bytes(file, name.as_str()).await?;
                lock.plugins_mut().insert(name.to_string(), {
                    ExtensionMeta::new(build, format!("./plugins/{}.jar", name))
                });
                lock.save().await
            }));
        }
        join_all(handler_list).await;
        Ok(())
    }
}

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = "./plugins/";
}
