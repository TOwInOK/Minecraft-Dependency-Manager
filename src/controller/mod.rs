use std::{sync::Arc, time::Duration};

use log::{error, info, trace};
use tokio::{sync::Mutex, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::{config::Config, downloader::Downloader, errors::error::Result, lock::locker::Lock};

const CONFIG_PATH: &str = "./config.toml";

pub struct Controller {
    config: Arc<Mutex<Config>>,
    lock: Arc<Mutex<Lock>>,
}

impl Controller {
    pub async fn init() {
        let controller = Self::new().await.unwrap();
        let token = CancellationToken::new();
        let token_clone = token.clone();

        let config = Arc::clone(&controller.config);
        let lock = Arc::clone(&controller.lock);
        tokio::spawn(async move {
            run(config, lock, &token).await;
        });

        watch_config_changes(&token_clone).await
    }

    async fn new() -> Result<Self> {
        // Load Config file
        let config = match Config::load_config(CONFIG_PATH).await {
            Ok(config) => Arc::new(Mutex::new(config)),
            Err(e) => {
                log::error!("Failed to load config: {}", e);
                log::warn!("Loading default config");
                Arc::new(Mutex::new(Config::default()))
            }
        };

        // Load lock
        let lock = {
            let mut lock = Lock::default();
            if let Err(e) = lock.load(&config.lock().await.additions.path_to_lock).await {
                error!("{}", e);
                lock.create(&config.lock().await.additions.path_to_lock)
                    .await?;
                lock.load(&config.lock().await.additions.path_to_lock)
                    .await?;
            }
            Arc::new(Mutex::new(lock))
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

async fn _watcher(token: &CancellationToken) {
    token.cancel();
    info!("Token Stopped {:#?}", token)
}

/// Check zombies entities.
/// Start download fn.
async fn start(config: &Arc<Mutex<Config>>, lock: &Arc<Mutex<Lock>>) {
    let config = config.lock().await;
    let mut lock = lock.lock().await;

    info!("Start removing useless things");
    if let Err(e) = remove_zombies(&mut lock, &config).await {
        error!("{:?}", e);
    }

    info!("Init downloader");
    if let Err(e) = Downloader::init(&config, &mut lock)
        .check_and_download()
        .await
    {
        error!("{:?}", e);
    }
}

async fn run(config: Arc<Mutex<Config>>, lock: Arc<Mutex<Lock>>, token: &CancellationToken) {
    // let sleep_cooldown = self.config.lock().await.additions.time_to_await;
    let cooldown = 100;
    loop {
        info!("Start checking and download");
        start(&config, &lock).await;

        // Sleep for 5 minutes
        tokio::select! {
            _ = sleep(Duration::from_millis(cooldown)) => {},
            _ = token.cancelled() => break,
        };
    }
}

/// Load downloader module.
/// Always check config file.
/// Use `token` for canceling minecraft task
pub async fn watch_config_changes(_token: &CancellationToken) {
    // Load new Config file
    let config = match Config::load_config(CONFIG_PATH).await {
        Ok(config) => {
            log::debug!("{:#?}", config);
            config
        }
        Err(e) => {
            log::error!("message: {}", e);
            log::warn!("Происходит загрузка стандартного конфига");
            Config::default()
        }
    };
    info!("Load Config successfully!");

    // Load new lock
    let mut lock = Lock::default();
    if let Err(e) = lock.load(&config.additions.path_to_lock).await {
        error!("{:?}", e);
        lock.create(&config.additions.path_to_lock)
            .await
            .unwrap_or_else(|e| error!("{:?}", e));
        lock.load(&config.additions.path_to_lock)
            .await
            .unwrap_or_else(|e| error!("{:?}", e));
    }
    info!("Load Lock successfuly!");
    // TODO: Add notify watcher
}
