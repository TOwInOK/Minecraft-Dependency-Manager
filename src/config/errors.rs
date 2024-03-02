use thiserror::Error;
#[derive(Error, Debug)]
pub enum DownloadErrors {
    #[error("Загрузка прекращенна потому что: {0}")]
    DownloadCorrapt(String),

}


#[derive(Error, Debug)]
pub enum ConfigErrors {
    #[error("Загрузка файла не была успешна: {0}")]
    LoadCorrapt(String),

}
