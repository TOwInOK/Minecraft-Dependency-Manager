
use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let name: String = "simple-voice-chat".to_string().to_lowercase();
    let version: String = "1.20.2".to_string();
    let core = "paper".to_string().to_lowercase();
    let actuality = "release".to_string().to_lowercase();
    let list = finder(&client, name.clone(), version.clone(), core.clone(), actuality.clone()).await;

    
    if let Some(list) = list {
        list.iter().for_each(|x| {
            println!("{:?}, {:?}, {:?}, {:?}", x.code, x.version, x.name, x.link);
        });
    }
    let one = finder_first(&client, name, version, core, actuality);
    if let Some(one) = one.await {
        println!("{:?}, {:?}, {:?}, {:?}", one.code, one.version, one.name, one.link);
    }else {
        println!("not found");
    }
}


///выдёт список ссылок плагинов на скачивание
async fn finder(client: &Client, name: String, version : String, core: String, actuality: String) -> Option<Vec<Plugin>> {
    let url = format!("https://modrinth.com/plugin/{}/versions?c={}&g={}&l={}", name, actuality, version, core);
    let response = client.get(url).send().await.expect("Failed to get the response");
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
    let mut url_list: Vec<Plugin> = Vec::new();
    for element in fragment.select(&selector) {
        let url: &str = element.value().attr("href")?;
        if url.starts_with("https://cdn.modrinth.com/data/") && element.value().attr("aria-label").is_some() {
            if url_list.iter().any(|x| x.link == url) {
                continue;
            }
            let label = element.value().attr("aria-label")?;
            url_list.push(Plugin{
                code: url.split("/").collect::<Vec<&str>>()[4].to_owned(),
                name: label[9..].to_owned(),
                link: url.to_owned(),
                version: url.split("/").collect::<Vec<&str>>()[6].to_owned(),
            });
        }
    }
    Some(url_list)
}


async fn finder_first(client: &Client, name: String, version : String, core: String, actuality: String) -> Option<Plugin> {
    let url = format!("https://modrinth.com/plugin/{}/versions?c={}&g={}&l={}", name, actuality, version, core);
    let response = client.get(url).send().await.expect("Failed to get the response");
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
    for element in fragment.select(&selector) {
        let url: &str = element.value().attr("href")?;
        if url.starts_with("https://cdn.modrinth.com/data/") && element.value().attr("aria-label").is_some() {
            let label = element.value().attr("aria-label")?;
            let plugin = Plugin{
                code: url.split("/").collect::<Vec<&str>>()[4].to_owned(),
                name: label[9..].to_owned(),
                link: url.to_owned(),
                version: url.split("/").collect::<Vec<&str>>()[6].to_owned(),
            };
            return Some(plugin);
        } else {
            return None;
        }
    }
    None
}

async fn chose() {
}


async fn download() {
}

struct Plugin {
    code: String,
    version: String,
    name: String,
    link: String,
}