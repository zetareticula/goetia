use std::collections::HashMap;
use crate::utils::extract::strip_remove_end;
use std::fmt;

const SCENE_MARKER: &str = "**Scene:** ";
const DESCRIPTION_MARKER: &str = "**Description:** ";
const STOP_MARKER: &str = "\n";
const END_MARKER: &str = "**END**";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub name: String,
    pub description: String,
    pub place: Option<String>,
    pub beat: Option<String>,
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
                place: None,
                beat: None,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Scenes {
    pub scene_descriptions: HashMap<String, String>,
    pub(crate) scenes: Vec<Scene>,
}

impl fmt::Display for Scenes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for scene in &self.scenes {
            writeln!(f, "{}{}\n{}{}", SCENE_MARKER, scene.name, DESCRIPTION_MARKER, scene.description)?;
        }
        Ok(())
    }
}

impl Scenes {
    pub fn new() -> Self {
        Self {
            scene_descriptions: HashMap::new(),
            scenes: Vec::new(),
        }
    }

    pub fn from_string(text: &str) -> Self {
        let mut scenes = Vec::new();
        let mut scene_descriptions = HashMap::new();

        // Split text into individual scene blocks
        let scene_blocks: Vec<&str> = text.split("**Scene:**").skip(1).collect();
        
        for block in scene_blocks {
            if let Some(scene) = Scene::from_string(&format!("**Scene:**{}", block)) {
                scenes.push(scene.clone());
                scene_descriptions.insert(scene.name.clone(), scene.description.clone());
            }
        }

        Self {
            scenes,
            scene_descriptions,
        }
    }
} 