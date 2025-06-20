use std::collections::HashMap;

use super::{Characters, Place, Scenes, Title};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Story {
    pub storyline: String,
    pub title: String,
    pub character_descriptions: HashMap<String, String>,
    pub place_descriptions: HashMap<String, Place>,
    pub scenes: Scenes,
    pub dialogs: Vec<String>,
}