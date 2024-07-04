use std::sync::Arc;
use std::time::Duration;

use log::error;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::errors::error::Result;
use crate::settings::additions::Additions;
use crate::SETTINGS;

pub async fn download(key: Arc<CancellationToken>) -> Result<()> {
    let duration = SETTINGS
        .read()
        .await
        .additions()
        .unwrap_or(&Additions::default())
        .duration()
        .unwrap_or(300);
    let cooldown = Duration::from_secs(duration);
    loop {
        '_core_scope: {
            tokio::spawn(async move {
                let settings = SETTINGS.read().await;
                settings.core().download().await
            });
        }
        '_plugins_scope: {
            tokio::spawn(async move {
                let settings = SETTINGS.read().await;
                if let Some(plugins) = settings.plugins() {
                    plugins
                        .download_all(
                            settings.core().provider().as_str(),
                            settings.core().version(),
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
