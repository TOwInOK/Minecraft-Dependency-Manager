use thiserror::Error;
#[derive(Error, Debug)]
pub enum DownloadErrors {
    #[error("Загрузка прекращена потому что: {0}")]
    DownloadCorrupt(String),
}

// Реализация From для преобразования std::io::Error в DownloadErrors
impl From<std::io::Error> for DownloadErrors {
    fn from(error: std::io::Error) -> Self {
        DownloadErrors::DownloadCorrupt(error.to_string())
    }
}

#[derive(Error, Debug)]
pub enum LockErrors {
    #[error("Ошибка удаление файла: {0}")]
    DeleteError(#[from] std::io::Error),
}

impl From<LockErrors> for DownloadErrors {
    fn from(error: LockErrors) -> Self {
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
    #[error("Ошибка сериализация TOML: {0}")]
    SerializeError(#[from] toml::ser::Error),
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
            ConfigErrors::SerializeError(msg) => DownloadErrors::DownloadCorrupt(msg.to_string()),
        }
    }
}

impl From<reqwest::Error> for ConfigErrors {
    fn from(value: reqwest::Error) -> Self {
        ConfigErrors::LoadCorrupt(value.to_string())
    }
}

impl From<reqwest::Error> for DownloadErrors {
    fn from(value: reqwest::Error) -> Self {
        DownloadErrors::DownloadCorrupt(value.to_string())
    }
}

#[derive(Error, Debug)]
pub enum CompareHashError {
    #[error("Конвертация Sha1 проведена не успешно : {0}")]
    SHA1(std::io::Error),
    #[error("Конвертация Sha256 проведена не успешно : {0}")]
    SHA256(std::io::Error),
    #[error("Конвертация Md5 проведена не успешно : {0}")]
    MD5(std::io::Error),
}

// Реализация From для преобразования DownloadErrors в ConfigErrors
impl From<CompareHashError> for ConfigErrors {
    fn from(value: CompareHashError) -> Self {
        match value {
            CompareHashError::SHA1(msg) => ConfigErrors::LoadCorrupt(msg.to_string()),
            CompareHashError::SHA256(msg) => ConfigErrors::LoadCorrupt(msg.to_string()),
            CompareHashError::MD5(msg) => ConfigErrors::LoadCorrupt(msg.to_string()),
        }
    }
}
