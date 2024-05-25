use std::sync::Arc;
use std::time::Duration;

use indicatif::MultiProgress;
use log::error;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::errors::error::Result;
use crate::settings::additions::Additions;
use crate::{lock::Lock, settings::Settings};

pub async fn download(
    settings: Arc<RwLock<Settings>>,
    lock: Arc<Mutex<Lock>>,
    mpb: Arc<MultiProgress>,
    key: Arc<CancellationToken>,
) -> Result<()> {
    let duraction = settings
        .read()
        .await
        .additions()
        .unwrap_or(&Additions::default())
        .duraction()
        .unwrap_or(300);
    let cooldown = Duration::from_secs(duraction);
    loop {
        '_core_scope: {
            let lock = Arc::clone(&lock);
            let settings = Arc::clone(&settings);
            let mpb = Arc::clone(&mpb);
            tokio::spawn(async move {
                let settings = settings.read().await;
                settings.core().download(lock, mpb).await
            });
        }
        '_plugins_scope: {
            let lock = Arc::clone(&lock);
            let settings = Arc::clone(&settings);
            let mpb = Arc::clone(&mpb);

            tokio::spawn(async move {
                let settings = settings.read().await;
                if let Some(plugins) = settings.plugins() {
                    plugins
                        .download_all(
                            settings.core().provider().as_str(),
                            settings.core().version(),
                            lock,
                            mpb,
                        )
                        .await
                        .map_err(|e| {
                            error!("Plugin scope error {:#?}", &e);
                            e
                        })
                } else {
                    Ok(())
                }
            });
        };
        tokio::select! {
            _ = sleep(cooldown) => {},
            _ = key.cancelled() => {break Ok(())},
        }
    }
}
