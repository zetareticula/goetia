use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::utils::extract::extract_elements;
use crate::utils::extract::strip_remove_end;

const SCENE_MARKER: &str = "**Scene:** ";
const DESCRIPTION_MARKER: &str = "**Description:** ";
const STOP_MARKER: &str = "\n";
const END_MARKER: &str = "**END**";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub name: String,
    pub description: String,
}

impl Scene {
    pub fn from_string(text: &str) -> Option<Self> {
        let elements: Vec<&str> = text.split(DESCRIPTION_MARKER).collect();
        if elements.len() == 2 {
            let name = elements[0].trim();
            let description = strip_remove_end(elements[1].trim());
            Some(Self {
                name: name.to_string(),
                description,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Scenes {
    pub scene_descriptions: HashMap<String, String>,
} 