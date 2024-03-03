use thiserror::Error;
#[derive(Error, Debug)]
pub enum DownloadErrors {
    #[error("Загрузка прекращенна потому что: {0}")]
    DownloadCorrupt(String),
}

// Реализация From для преобразования std::io::Error в DownloadErrors
impl From<std::io::Error> for DownloadErrors {
    fn from(error: std::io::Error) -> Self {
        DownloadErrors::DownloadCorrupt(error.to_string())
    }
}

#[derive(Error, Debug)]
pub enum ConfigErrors {
    #[error("Загрузка файла не была успешна: {0}")]
    LoadCorrupt(String),
    #[error("Ошибка чтения файла: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Ошибка парсинга TOML: {0}")]
    ParseError(#[from] toml::de::Error),
}

// Реализация From для преобразования DownloadErrors в ConfigErrors
impl From<DownloadErrors> for ConfigErrors {
    fn from(value: DownloadErrors) -> Self {
        match value {
            DownloadErrors::DownloadCorrupt(msg) => ConfigErrors::LoadCorrupt(msg),
        }
    }
}

// Реализация From для преобразования ConfigErrors в DownloadErrors
impl From<ConfigErrors> for DownloadErrors {
    fn from(value: ConfigErrors) -> Self {
        match value {
            ConfigErrors::LoadCorrupt(msg) => DownloadErrors::DownloadCorrupt(msg),
            ConfigErrors::ReadError(msg) => DownloadErrors::DownloadCorrupt(msg.to_string()),
            ConfigErrors::ParseError(msg) => DownloadErrors::DownloadCorrupt(msg.to_string()),
        }
    }
}

impl From<reqwest::Error> for ConfigErrors {
    fn from(value: reqwest::Error) -> Self {
        ConfigErrors::LoadCorrupt(value.to_string())
    }
}
