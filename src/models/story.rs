use std::collections::HashMap;

use super::{Place, Scenes};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Story {
    pub storyline: String,
    pub title: String,
    pub character_descriptions: HashMap<String, String>,
    pub place_descriptions: HashMap<String, Place>,
    pub scenes: Scenes,
    pub dialogs: Vec<String>,
}

impl Story {
    pub fn new(storyline: String) -> Self {
        Self {
            storyline,
            title: String::new(),
            character_descriptions: HashMap::new(),
            place_descriptions: HashMap::new(),
            scenes: Scenes::new(),
            dialogs: Vec::new(),
        }
    }
}