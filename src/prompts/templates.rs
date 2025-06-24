use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PromptTemplates {
    templates: HashMap<String, String>,
}

impl PromptTemplates {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
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