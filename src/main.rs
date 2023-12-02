use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs;

#[tokio::main]
async fn main() {
    //Http agent
    let client = reqwest::Client::new();

    //Load Config file
    let path = "./config.toml".to_string();
    let config = load_config(path);

    // //Metadata for plugins to download
    // let data = MetaData {
    //     name: "simple-voice-chat".to_string().to_lowercase(),
    //     version: "1.20.2".to_string(),
    //     core: "paper".to_string().to_lowercase(),
    //     actuality: "release".to_string().to_lowercase(),
    // };

    // //List of links for download
    // let list = finder(&client, data.clone()).await;

    // if let Some(list) = list {
    //     list.iter().for_each(|x| {
    //         println!("{:?}, {:?}, {:?}, {:?}", x.code, x.version, x.name, x.link);
    //     });
    // }

    // let one = finder_first(&client, data).await;
    // if let Some(one) = one{
    //     println!("{:?}, {:?}, {:?}, {:?}", one.code, one.version, one.name, one.link);
    // }else {
    //     println!("not found");
    // }
}

/// выдёт список ссылок плагинов на скачивание
/// {```Code, Version, Name, Link```}
async fn finder(client: &Client, data: MetaData) -> Option<Vec<PluginInfo>> {
    let url = format!(
        "https://modrinth.com/{}/{}/versions?c={}&g={}&l={}",
        data.is, data.name, data.actuality, data.version, data.core
    );
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to get the response");
    let body = match response.text_with_charset("utf-8").await {
        Ok(body) => body,
        Err(_) => {
            return None;
        }
    };
    let fragment = Html::parse_document(&body);
    let selector = match Selector::parse(r"#all-versions a:not(style)") {
        Ok(selector) => selector,
        Err(_) => return None,
    };
    let mut url_list: Vec<PluginInfo> = Vec::new();
    for element in fragment.select(&selector) {
        let url: &str = element.value().attr("href")?;
        if url.starts_with("https://cdn.modrinth.com/data/")
            && element.value().attr("aria-label").is_some()
        {
            if url_list.iter().any(|x| x.link == url) {
                continue;
            }
            let label = element.value().attr("aria-label")?;
            url_list.push(PluginInfo {
                code: url.split('/').collect::<Vec<&str>>()[4].to_owned(),
                name: label[9..].to_owned(),
                link: url.to_owned(),
                version: url.split('/').collect::<Vec<&str>>()[6].to_owned(),
            });
        }
    }
    Some(url_list)
}

///Поиск плагина и вывод его одиночной актуальной версии
async fn finder_first(client: &Client, data: MetaData) -> Option<PluginInfo> {
    let url = format!(
        "https://modrinth.com/{}/{}/versions?c={}&g={}&l={}",
        data.is, data.name, data.actuality, data.version, data.core
    );
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to get the response");
    let body = match response.text_with_charset("utf-8").await {
        Ok(body) => body,
        Err(_) => {
            return None;
        }
    };
    let fragment = Html::parse_document(&body);
    let selector = match Selector::parse(r"#all-versions a:not(style)") {
        Ok(selector) => selector,
        Err(_) => return None,
    };
    if let Some(element) = fragment.select(&selector).next() {
        let url: &str = element.value().attr("href")?;
        if url.starts_with("https://cdn.modrinth.com/data/")
            && element.value().attr("aria-label").is_some()
        {
            let label = element.value().attr("aria-label")?;
            let plugin = PluginInfo {
                code: url.split('/').collect::<Vec<&str>>()[4].to_owned(),
                name: label[9..].to_owned(),
                link: url.to_owned(),
                version: url.split('/').collect::<Vec<&str>>()[6].to_owned(),
            };
            return Some(plugin);
        } else {
            return None;
        }
    }
    None
}

/// Отдаём конфиг и скачиаем
async fn catcher(client: Client, config: Config) {
    let plugins: Vec<String> = {
        match config.plugins {
            Some(p) => {
                let mut plugin_list: Vec<String> = Vec::new();
                // Создаём более интуитивный вектор
                let plugin_collections = vec![p.modrinth, p.paper, p.spigot];
                //Если есть frozen, то проходимся по нему и удаляем названия
                if let Some(f) = p.frozen {
                    for c in plugin_collections.into_iter().flatten() {
                        check_frozen(&mut plugin_list, &f, c).await;
                    }
                }
                // Если нет frozen, то просто проходимся и удаляем названия
                else {
                    for c in plugin_collections.into_iter().flatten() {
                        plugin_list.extend(c)
                    }
                }
                plugin_list
            }
            None => {
                println!("Конфигурация плагинов не была обнаружена");
                let plugin_list: Vec<String> = Vec::new();
                plugin_list
            }
        }
    };
    let datapacks: Vec<String> = {
        match config.datapacks {
            Some(d) => {
                let mut datapack_list: Vec<String> = Vec::new();
                let datapack_collections = vec![d.modrinth, d.paper, d.spigot];
                //Если есть frozen, то проходимся по нему удаляя и добавляя названия
                if let Some(f) = d.frozen {
                    for c in datapack_collections.into_iter().flatten() {
                        check_frozen(&mut datapack_list, &f, c).await;
                    }
                }
                // Если нет frozen, то просто проходимся добавляя названия
                else {
                    for c in datapack_collections.into_iter().flatten() {
                        datapack_list.extend(c)
                    }
                }
                datapack_list
            }
            None => {
                println!("Конфигурация датапаков не была обнаружена");
                let datapack_list: Vec<String> = Vec::new();
                datapack_list
            }
        }
    };
    todo!("Скачиваем ссылки из download")
}

async fn download() {
    todo!()
}

async fn check_frozen(
    list_to_add: &mut Vec<String>,
    words_to_remove: &[String],
    dirty_list: Vec<String>,
) {
    // Создаем временный вектор для хранения слов, которые нужно оставить
    let mut temp_vec: Vec<String> = Vec::new();

    // Проверяем каждое слово в plugin_list
    for word in dirty_list.iter() {
        // Если слово не содержится в words_to_remove, добавляем его в temp_vec
        if !words_to_remove.contains(word) {
            temp_vec.push(word.clone());
        }
    }
    list_to_add.extend(temp_vec);
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

struct PluginInfo {
    code: String,
    version: String,
    name: String,
    link: String,
}

struct DatapackInfo {
    code: String,
    version: String,
    name: String,
    link: String,
}

struct MetaData {
    name: String,
    core: String,
    version: String,
    actuality: String,
    is: String,
}

#[derive(Deserialize, Serialize)]
struct Config {
    version: Version,
    plugins: Option<Plugin>,
    datapacks: Option<Datapack>,
}

#[derive(Deserialize, Serialize)]
struct Version {
    core: String,
    version: String,
    frozen: Option<bool>,
}

#[derive(Deserialize, Serialize)]
struct Plugin {
    modrinth: Option<Vec<String>>,
    spigot: Option<Vec<String>>,
    paper: Option<Vec<String>>,
    frozen: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize)]
struct Datapack {
    modrinth: Option<Vec<String>>,
    spigot: Option<Vec<String>>,
    paper: Option<Vec<String>>,
    frozen: Option<Vec<String>>,
}
