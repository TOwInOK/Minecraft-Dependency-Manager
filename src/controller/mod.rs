use std::{error::Error, time::Duration};

use log::{error, info, trace};
use tokio::{sync::Mutex, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::{
    config::Config, downloader::Downloader, errors::error::LockErrors, lock::locker::Lock,
};

pub struct Controller {
    config: Mutex<Config>,
    lock: Mutex<Lock>,
}

impl Controller {
    pub async fn init() {
        let controller = Self::new().await;
        controller.watch_config_changes().await;
    }

    async fn new() -> Self {
        // Load Config file
        let path = "./config.toml";
        let mut config = Config::load_config(path).await.unwrap_or_else(|e| {
            log::error!("message: {}", e);
            log::warn!("Происходит загрузка стандартного конфига");
            Config::default()
        });
        config.additions.path_to_configs = path.to_owned();

        // Load lock
        let mut lock = Lock::default();
        if let Err(e) = lock.load(&config.additions.path_to_lock).await {
            error!("{e}");
            lock.create(&config.additions.path_to_lock).await.unwrap();
            lock.load(&config.additions.path_to_lock).await.unwrap();
        }

        let lock = Mutex::new(lock);
        let config = Mutex::new(config);

        Self { config, lock }
    }

    async fn run(&mut self, token: CancellationToken) {
        // let sleep_cooldown = self.config.lock().await.additions.time_to_await;
        let cooldown = 100;
        loop {
            info!("Start checking and download");
            self.start().await;

            // Sleep for 5 minutes
            tokio::select! {
                _ = sleep(Duration::from_millis(cooldown)) => {},
                _ = token.cancelled() => break,
            };
        }
    }

    /// Check zombies entities.
    /// Start download fn.
   async fn start(&self) {
        let mut config = self.config.lock().await;
        let mut lock = self.lock.lock().await;

        info!("Start removing useless things");
        if let Err(e) = remove_zombies(&mut lock, &mut config).await {
            error!("{:?}", e);
        }

        info!("Init downloader");
        if let Err(e) = Downloader::init(&mut config, &mut lock)
            .check_and_download()
            .await
        {
            error!("{:?}", e);
        }
    }

    pub async fn watch_config_changes(&self) {
        let token = CancellationToken::new();
        let downloader = tokio::spawn(self.run(token));

        // Load new Config file
        let path = self.config.lock().await.additions.path_to_configs.to_owned();
        let config = match Config::load_config(&path).await {
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

        // Load new lock
        let mut lock = Lock::default();
        if let Err(e) = lock.load(&config.additions.path_to_configs).await {
            error!("{:?}", e);
            lock.create(&config.additions.path_to_configs)
                .await
                .unwrap();
            lock.load(&config.additions.path_to_configs)
                .await
                .unwrap_or_else(|e| error!("{:?}", e));
        }

        *self.lock.lock().await = lock;
        *self.config.lock().await = config;

        watcher(token).await;
    }
}

/// Итерируем Lock и находим то чего нет в Config.
/// Нет, удаляем в Lock.
async fn remove_zombies(lock: &mut Lock, config: &Config) -> Result<(), LockErrors> {
    trace!("Start fn: remove_zombies");
    lock.remove_if_not_exist_plugin(config).await?;
    lock.remove_if_not_exist_core(config).await
}


async fn watcher(token: CancellationToken) {
    token.cancel();
    info!("Token Stopped {:#?}", token)
}