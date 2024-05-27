use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::mode::Mode;

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Mods(HashMap<String, Mode>);
