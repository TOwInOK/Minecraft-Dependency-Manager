use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use async_watcher::{notify::RecursiveMode, AsyncDebouncer};
use log::{error, info, trace};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::{config::Config, downloader::Downloader, errors::error::Result, lock::locker::Lock};

const CONFIG_PATH: &str = "./config.toml";

pub struct Controller {
    config: Config,
    lock: Lock,
}

impl Controller {
    pub fn init() -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            let mut controller = Self::new().await.unwrap();

            info!("Start removing useless things");
            if let Err(e) = remove_zombies(&mut controller.lock, &controller.config).await {
                error!("{:?}", e);
            }
            let token = Arc::new(CancellationToken::new());
            let token_clone = token.clone();

            let checker = tokio::spawn(async move {
                run(&controller.config, &mut controller.lock, &token).await;
            });
            let finder = tokio::spawn(async move {
                watch_config_changes(&token_clone).await;
            });
            let (_, _) = tokio::try_join!(checker, finder).unwrap();
        })
    }

    async fn new() -> Result<Self> {
        // Load Config file
        let config = match Config::load_config(CONFIG_PATH).await {
            Ok(config) => config,
            Err(e) => {
                log::error!("Failed to load config: {}", e);
                log::warn!("Loading default config");
                Config::default()
            }
        };

        // Load lock
        let lock = {
            let mut lock = Lock::default();
            if let Err(e) = lock.load(&config.additions.path_to_lock).await {
                error!("{}", e);
                lock.create(&config.additions.path_to_lock).await?;
                lock.load(&config.additions.path_to_lock).await?;
            }
            lock
        };

        Ok(Self { config, lock })
    }
}

/// Итерируем Lock и находим то чего нет в Config.
/// Нет, удаляем в Lock.
async fn remove_zombies(lock: &mut Lock, config: &Config) -> Result<()> {
    trace!("Start fn: remove_zombies");
    lock.remove_if_not_exist_plugin(config).await?;
    lock.remove_if_not_exist_core(config).await
}

async fn run(config: &Config, lock: &mut Lock, token: &CancellationToken) {
    // let sleep_cooldown = self.config.lock().await.additions.time_to_await;
    let cooldown = 600f32;
    loop {
        info!("Start checking and download");
        info!("Init downloader");
        if let Err(e) = Downloader::init(config, lock).check_and_download().await {
            error!("{:?}", e);
        }

        // Sleep for 5 minutes
        tokio::select! {
            _ = sleep(Duration::from_secs_f32(cooldown)) => {},
            _ = token.cancelled() => break,
        };
    }
}

/// Load downloader module.
/// Always check config file.
/// Use `token` for canceling minecraft task
pub async fn watch_config_changes(token: &CancellationToken) {
    trace!("Start Watch Config");
    // initialize the debouncer
    let (mut tx, mut rx) = AsyncDebouncer::new_with_channel(Duration::from_millis(200), None)
        .await
        .unwrap();
    info!("Get debouncer");
    // register path to watch
    tx.watcher()
        .watch(CONFIG_PATH.as_ref(), RecursiveMode::NonRecursive)
        .unwrap();
    info!("Fill debouncer");
    // wait for events
    while let Some(event) = rx.recv().await {
        trace!("event: {:?}", event);
        token.cancel();
        Controller::init().await
    }
}
