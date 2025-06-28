use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::utils::extract::extract_elements;

const CHARACTER_MARKER: &str = "**Character:** ";
const DESCRIPTION_MARKER: &str = "**Description:** ";
const STOP_MARKER: &str = "\n";
const END_MARKER: &str = "**END**";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character {
    pub name: String,
    pub description: String,
}

impl Character {
    pub fn from_string(text: &str) -> Option<Self> {
        let elements: Vec<&str> = text.split(DESCRIPTION_MARKER).collect();
        if elements.len() == 2 {
            let name = elements[0].trim();
            let description = elements[1].trim();
            Some(Self {
                name: name.to_string(),
                description: description.to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Characters {
    pub character_descriptions: HashMap<String, String>,
}

impl Characters {
    pub fn new() -> Self {
        Self {
            character_descriptions: HashMap::new()
        }
    }

    pub fn from_string(text: &str) -> Self {
        let mut character_descriptions = HashMap::new();
        let elements = extract_elements(text, CHARACTER_MARKER, STOP_MARKER);
        for text_character in elements {
            if let Some(character) = Character::from_string(&text_character) {
                character_descriptions.insert(character.name, character.description);
            }
        }
        Self { character_descriptions }
    }
}

impl fmt::Display for Characters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("\n");
        for (name, description) in &self.character_descriptions {
            s.push_str(&format!(
                "\n{}{} {}{} {}\n",
                CHARACTER_MARKER, name, DESCRIPTION_MARKER, description, STOP_MARKER
            ));
        }
        s.push_str(END_MARKER);
        write!(f, "{}", s)
    }
}

impl FromStr for Characters {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_string(s))
    }
}