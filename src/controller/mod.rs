use std::time::Duration;

use log::{error, info};
use tokio::{sync::Mutex, time::sleep};

use crate::{config::Config, downloader::Downloader, lock::lock::Lock};

pub struct Controller {
    config: Mutex<Config>,
    lock: Mutex<Lock>,
}

impl Controller {
    pub async fn init() -> Self {
        let mut controller = Self::new().await;
        controller.run().await;
        controller
    }

    async fn new() -> Self {
        // Load Config file
        let path =
            "/Users/dmitryfefilov/Documents/Rust/MinecraftAddonController/config.toml".to_string();
        let config = Config::load_config(path).await.unwrap_or_else(|e| {
            log::error!("message: {}", e);
            log::warn!("Происходит загрузка стандартного конфига");
            Config::default()
        });

        // Load lock
        let mut lock = Lock::default();
        if let Err(e) = lock.load(&config.additions.path_to_configs).await {
            error!("{e}");
            lock.create(&config.additions.path_to_configs)
                .await
                .unwrap();
            lock.load(&config.additions.path_to_configs).await.unwrap();
        }

        let lock = Mutex::new(lock);
        let config = Mutex::new(config);

        Self { config, lock }
    }

    async fn run(&mut self) {
        // let sleep_cooldown = self.config.lock().await.additions.time_to_await;
        let cooldown = 100;
        loop {
            info!("Start checking and download");
            self.start().await;

            // Sleep for 5 minutes
            sleep(Duration::from_secs(cooldown)).await;
        }
    }

    async fn start(&mut self) {
        let config = self.config.get_mut();
        let lock = self.lock.get_mut();

        Downloader::new(config, lock)
            .check_and_download()
            .await
            .unwrap_or_else(|e| error!("{e}"));
    }

    pub async fn watch_config_changes(&mut self) {
        // Load new Config file
        let path = self.config.lock().await.additions.path_to_configs.clone();
        let config = Config::load_config(path).await.unwrap_or_else(|e| {
            log::error!("message: {}", e);
            log::warn!("Происходит загрузка стандартного конфига");
            Config::default()
        });
        log::debug!("{:#?}", config);

        // Load new lock
        let mut lock = Lock::default();
        if let Err(e) = lock.load(&config.additions.path_to_configs).await {
            error!("{e}");
            lock.create(&config.additions.path_to_configs)
                .await
                .unwrap();
            lock.load(&config.additions.path_to_configs)
                .await
                .unwrap_or_else(|e| error!("{e}"));
        }
        self.lock = lock.into();
        self.config = config.into();
    }
}
