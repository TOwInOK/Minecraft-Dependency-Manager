use serde::{Deserialize, Serialize};

///Lists of plugins
#[derive(Deserialize, Serialize, Debug)]
pub struct Plugin {
    //list to download from https://modrinth.com/
    modrinth: Option<Vec<String>>,
    //list to download from https://www.spigotmc.org/
    spigot: Option<Vec<String>>,
    //list to download from https://hangar.papermc.io/
    paper: Option<Vec<String>>,
    //List of plugins to stop updating
    frozen: Option<Vec<String>>,
}

impl Plugin {
    fn new(
        modrinth: Option<Vec<String>>,
        spigot: Option<Vec<String>>,
        paper: Option<Vec<String>>,
        frozen: Option<Vec<String>>,
    ) -> Self {
        Self {
            modrinth,
            spigot,
            paper,
            frozen,
        }
    }
    pub fn default() -> Self {
        Plugin::new(None, None, None, None)
    }
}
