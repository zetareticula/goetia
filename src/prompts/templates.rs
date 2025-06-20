
//! This module provides a set of predefined prompt templates for generating story elements
//! //! It includes templates for titles, characters, scenes, settings, and dialog.
//! //! The templates are designed to be used with a language model to generate creative content.
//! //! The templates are stored in a `HashMap` for easy retrieval by key.
//! //! # Example
//! //! ```
//! //! use crate::prompts::templates::PromptTemplates;
//! //! //! let templates = PromptTemplates::new();
//! //! //! // Retrieve a specific template
//! //! //! let title_prompt = templates.get("TITLES_PROMPT").unwrap();
//! //! //! println!("{}", title_prompt);
//! //! //! // Output: "Generate a title for the following storyline:\n"
//! //! //! let characters_prompt = templates.get("CHARACTERS_PROMPT").unwrap();
//! //! //! println!("{}", characters_prompt);
//! //! //! // Output: "Generate characters for the following storyline:\n"
//! //! //! let scene_prompt = templates.get("SCENE_PROMPT").unwrap();
//! //! //! println!("{}", scene_prompt);
//! //! //! // Output: "Generate scenes for the following storyline and characters:\n"

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Visitor};   
use std::fmt;
use std::str::FromStr;
use crate::utils::extract::extract_elements;

// Define a struct to hold the prompt templates
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
#[serde(default)]
#[serde(try_from = "String")]
#[serde(into = "String")]
#[serde(serialize_with = "serialize_templates")]
#[serde(deserialize_with = "deserialize_templates")]
pub struct PromptTemplates {
    templates: HashMap<String, String>,
}

// Custom serialization function for PromptTemplates
fn serialize_templates<S>(templates: &PromptTemplates, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut map = serializer.serialize_map(Some(templates.templates.len()))?;
    for (key, value) in &templates.templates {
        map.serialize_entry(key, value)?;
    }
    map.end()
}

// Custom deserialization function for PromptTemplates
fn deserialize_templates<'de, D>(deserializer: D) -> Result<PromptTemplates, D::Error>
where
    D: Deserializer<'de>,
{
    struct PromptTemplatesVisitor;

    impl<'de> Visitor<'de> for PromptTemplatesVisitor {
        type Value = PromptTemplates;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of prompt templates")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut templates = HashMap::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                templates.insert(key, value);
            }
            Ok(PromptTemplates { templates })
        }
    }

    deserializer.deserialize_map(PromptTemplatesVisitor)
}



#[derive(Debug, Clone)]
pub struct PromptTemplates {
    templates: HashMap<String, String>,
}

//// This module provides a set of predefined prompt templates for generating story elements
impl PromptTemplates {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        // Placeholder prompts inferred from notebook intent
        templates.insert(
            "TITLES_PROMPT".to_string(),
            "Generate a title for the following storyline:\n".to_string(),
        );
        templates.insert(
            "CHARACTERS_PROMPT".to_string(),
            "Generate characters for the following storyline:\n".to_string(),
        );
        templates.insert(
            "SCENE_PROMPT".to_string(),
            "Generate scenes for the following storyline and characters:\n".to_string(),
        );
        templates.insert(
            "SETTING_PROMPT".to_string(),
            "Generate place descriptions for the following storyline:\n".to_string(),
        );
        templates.insert(
            "DIALOG_PROMPT".to_string(),
            "Generate dialog for the following scene:\n".to_string(),
        );
        Self { templates }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.templates.get(key)
    }
}