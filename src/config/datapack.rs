use serde::{Deserialize, Serialize};

///Lists of Datapacks
#[derive(Deserialize, Serialize, Debug)]
pub struct Datapack {
    //list to download from https://modrinth.com/
    modrinth: Option<Vec<String>>,
    //list to download from https://www.spigotmc.org/
    spigot: Option<Vec<String>>,
    //list to download from https://hangar.papermc.io/
    paper: Option<Vec<String>>,
    //List of plugins to stop updating
    frozen: Option<Vec<String>>,
}