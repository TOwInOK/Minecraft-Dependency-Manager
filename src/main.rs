use serde::{Deserialize, Serialize};
use std::fs;
mod model;
use model::*;

#[tokio::main]
async fn main() {
    //Http agent
    let client = reqwest::Client::new();

    //Load Config file
    let path = "./config.toml".to_string();
    let config = load_config(path);

}

async fn load_config(path: String) -> Config {
    let default: Config = Config {
        version: Version {
            core: "paper".to_string(),
            version: "1.20.1".to_string(),
            frozen: Some(false),
        },
        plugins: None,
        datapacks: None,
    };

    let toml = {
        println!("Загрузка конфигурационного файла...");
        let result = fs::read_to_string(path);
        match result {
            Ok(content) => {
                println!("Файл успешно загружен.");
                content
            }
            Err(_) => {
                println!(
                    "Ваш конфигурационный файл не был обнаружен, загружаю стандартные настройки"
                );
                toml::to_string(&default).expect("Failed to serialize default config to TOML")
            }
        }
    };
    drop(default);

    let config: Config = match toml::from_str(&toml) {
        Ok(parsed_config) => {
            println!("Конфигурация успешно загружена.");
            parsed_config
        }
        Err(e) => {
            println!("Не удалось загрузить конфигурацию, использую настройки по умолчанию.\n{e}");
            toml::from_str(
                r#"
                [main]
                core = "paper"
                version = "1.20.2"
            "#,
            )
            .expect("Failed to deserialize default config from TOML")
        }
    };
    config
}

///Struct to load config from toml file.
#[derive(Deserialize, Serialize)]
struct Config {
    version: Version,
    plugins: Option<Plugin>,
    datapacks: Option<Datapack>,
}


///Settings of server core
#[derive(Deserialize, Serialize)]
struct Version {
    //version of core
    core: String,
    //version
    version: String,
    //stop update this version of core
    frozen: Option<bool>,
}

///Lists of plugins
#[derive(Deserialize, Serialize)]
struct Plugin {
    //list to download from https://modrinth.com/
    modrinth: Option<Vec<String>>,
    //list to download from https://www.spigotmc.org/
    spigot: Option<Vec<String>>,
    //list to download from https://hangar.papermc.io/
    paper: Option<Vec<String>>,
    //List of plugins to stop updating
    frozen: Option<Vec<String>>,
}
///Lists of Datapacks
#[derive(Deserialize, Serialize)]
struct Datapack {
    //list to download from https://modrinth.com/
    modrinth: Option<Vec<String>>,
    //list to download from https://www.spigotmc.org/
    spigot: Option<Vec<String>>,
    //list to download from https://hangar.papermc.io/
    paper: Option<Vec<String>>,
    //List of plugins to stop updating
    frozen: Option<Vec<String>>,
}
