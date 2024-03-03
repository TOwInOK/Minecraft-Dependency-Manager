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

impl Datapack {
    // fn new(
    //     modrinth: Option<Vec<String>>,
    //     spigot: Option<Vec<String>>,
    //     paper: Option<Vec<String>>,
    //     frozen: Option<Vec<String>>,
    // ) -> Self {
    //     Self {
    //         modrinth,
    //         spigot,
    //         paper,
    //         frozen,
    //     }
    // }
    // pub fn default() -> Self {
    //     Datapack::new(None, None, None, None)
    // }
}
