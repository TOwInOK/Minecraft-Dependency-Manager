use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompareHashError {
    #[error("Хэш не совпадает")]
    HashNotCompare(),
    #[error("Конвертация Sha1 проведена не успешно : {0}")]
    SHA1(std::io::Error),
    #[error("Конвертация Sha256 проведена не успешно : {0}")]
    SHA256(std::io::Error),
    #[error("Конвертация Md5 проведена не успешно : {0}")]
    MD5(std::io::Error),
    #[error("Хэш отсутствует")]
    None,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Проблема с обработкой запроса: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Проблема с обработкой хэша: {0}")]
    CompareHash(CompareHashError),
    // #[error("Проблема с скачиванием файла: {0}")]
    // Download(String),
    #[error("Ошибка ввода/вывода: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ошибка парсинга TOML: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("Ошибка сериализация TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Не удалось найти: {0}")]
    NotFound(String),
    // #[error("Ошибка: {0}")]
    // Any(#[from] Box<dyn std::error::Error>),
}
pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! not_found {
    ($msg:expr) => {
        Err(Error::NotFound($msg.to_string()))
    };
}

#[macro_export]
macro_rules! not_found_path {
    ($arg:expr) => {
        Err(Error::NotFound(format!(
            "No path like: ->{}<-, exist",
            $arg
        )))
    };
}

// #[macro_export]
// macro_rules! download_error {
//     ($($arg:tt)*) => {
//         Err(Error::Download(format!($($arg)*)))
//     };
// }

#[macro_export]
macro_rules! not_found_build_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!(
            "No one build like: ->{}<- find",
            $arg
        )))
    };
}

#[macro_export]
macro_rules! not_found_version_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!("No one version ->{}<- find", $arg)))
    };
}

#[macro_export]
macro_rules! not_found_plugin_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!(
            "No one plugin: ->{}<-, has found.",
            $arg
        )))
    };
}
