use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;

use crate::api::{FilterApi, LanguageApi};
use crate::config::hyperparameters::Hyperparameters;
use crate::models::{Characters, Place, Scenes, Story, Title};
use crate::prompts::templates::PromptTemplates;
use crate::utils::diff::{
    diff_prompt_change_dict, diff_prompt_change_scenes, diff_prompt_change_str,
};
use crate::utils::extract::strip_remove_end;

use super::text_generator::{TextGenerationError, TextGenerator};

#[derive(Debug, thiserror::Error)]
pub enum StoryGenerationError {
    #[error("Text generation error: {0}")]
    TextGeneration(#[from] TextGenerationError),
    #[error("Invalid level: {0}")]
    InvalidLevel(usize),
    #[error("Invalid scene index: {0}")]
    InvalidSceneIndex(i32),
    #[error("Missing prompt: {0}")]
    MissingPrompt(String),
    #[error("Place not found: {0}")]
    PlaceNotFound(String),
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub const LEVEL_NAMES: [&str; 6] = [
    "storyline",
    "title",
    "characters",
    "scenes",
    "places",
    "dialogs",
];

#[derive(Debug, Clone)]
pub struct StoryGenerator {
    storyline: String,
    prefixes: PromptTemplates,
    hyperparameters: Hyperparameters,
    text_generator: TextGenerator,
    prompts: HashMap<String, Vec<String>>,
    title: Title,
    characters: Characters,
    scenes: Scenes,
    places: HashMap<String, Place>,
    dialogs: Vec<String>,
    interventions: HashMap<f64, String>,
    level: usize,
}

impl StoryGenerator {
    pub fn new(
        storyline: String,
        prefixes: PromptTemplates,
        hyperparameters: Hyperparameters,
        text_generator: TextGenerator,
    ) -> Self {
        let mut prompts = HashMap::new();
        prompts.insert("title".to_string(), vec!["".to_string()]);
        prompts.insert("characters".to_string(), vec!["".to_string()]);
        prompts.insert("scenes".to_string(), vec!["".to_string()]);
        prompts.insert("places".to_string(), vec!["".to_string()]);
        prompts.insert("dialogs".to_string(), vec!["".to_string()]);

        let mut generator = Self {
            storyline: storyline.clone(),
            prefixes,
            hyperparameters: hyperparameters.clone(),
            text_generator,
            prompts,
            title: Title::new(String::new()),
            characters: Characters {
                character_descriptions: HashMap::new(),
            },
            scenes: Scenes { scenes: vec![] },
            places: HashMap::new(),
            dialogs: vec!["".to_string()],
            interventions: HashMap::new(),
            level: 0,
        };
        generator.set_storyline(storyline);
        generator
    }

    fn set_storyline(&mut self, storyline: String) {
        self.level = 0;
        let storyline = if !storyline.ends_with('.') {
            format!("{}.", storyline)
        } else {
            storyline
        };
        self.storyline = storyline.clone();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.interventions
            .insert(timestamp, format!("STORYLINE\n{}", storyline));
    }

    pub fn seed(&self) -> u64 {
        self.hyperparameters.default_seed
    }

    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn characters(&self) -> &Characters {
        &self.characters
    }

    pub fn scenes(&self) -> &Scenes {
        &self.scenes
    }

    pub fn places(&self) -> &HashMap<String, Place> {
        &self.places
    }

    pub fn dialogs(&self) -> &[String] {
        &self.dialogs
    }

    pub fn title_str(&self) -> String {
        self.title.title.clone()
    }

    pub fn num_scenes(&self) -> usize {
        self.scenes.scenes.len()
    }

    pub async fn generate_title(
        &self,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(Title, String), StoryGenerationError> {
        let titles_prefix = format!(
            "{}{} Title: ",
            self.prefixes
                .get("TITLES_PROMPT")
                .ok_or_else(|| StoryGenerationError::MissingPrompt("TITLES_PROMPT".to_string()))?,
            self.storyline
        );
        let title_text = self
            .text_generator
            .generate_text_no_loop(
                &titles_prefix,
                client,
                filter,
                Some(self.hyperparameters.sample_length_title),
                seed,
                num_samples,
            )
            .await?;
        let title = Title::from_string(&format!("Title: {}", title_text));
        Ok((title, titles_prefix))
    }

    pub async fn generate_characters(
        &self,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(Characters, String), StoryGenerationError> {
        let characters_prefix = format!(
            "{}{}",
            self.prefixes.get("CHARACTERS_PROMPT").ok_or_else(|| {
                StoryGenerationError::MissingPrompt("CHARACTERS_PROMPT".to_string())
            })?,
            self.storyline
        );
        let characters_text = self
            .text_generator
            .generate_text(
                &characters_prefix,
                client,
                filter,
                None,
                self.hyperparameters.max_paragraph_length_characters,
                seed,
                num_samples,
                Some(self.hyperparameters.max_num_repetitions),
            )
            .await?;
        let characters = Characters::from_string(&characters_text);
        Ok((characters, characters_prefix))
    }

    pub async fn generate_scenes(
        &self,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(Scenes, String), StoryGenerationError> {
        let mut scenes_prefix = format!(
            "{}{}\n",
            self.prefixes
                .get("SCENE_PROMPT")
                .ok_or_else(|| StoryGenerationError::MissingPrompt("SCENE_PROMPT".to_string()))?,
            self.storyline
        );
        for (name, description) in &self.characters.character_descriptions {
            scenes_prefix.push_str(&format!("{}: {}\n", name, description));
        }
        scenes_prefix.push_str("\n**Scenes:**");
        let scenes_text = self
            .text_generator
            .generate_text(
                &scenes_prefix,
                client,
                filter,
                None,
                self.hyperparameters.max_paragraph_length_scenes,
                seed,
                num_samples,
                Some(self.hyperparameters.max_num_repetitions),
            )
            .await?;
        let scenes = Scenes::from_string(&scenes_text);
        Ok((scenes, scenes_prefix))
    }

    pub async fn generate_place_descriptions(
        &self,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(HashMap<String, Place>, Vec<String>), StoryGenerationError> {
        let mut place_descriptions = HashMap::new();
        let mut place_prefixes = Vec::new();
        let place_prefix = format!(
            "{}{}\n",
            self.prefixes
                .get("SETTING_PROMPT")
                .ok_or_else(|| StoryGenerationError::MissingPrompt("SETTING_PROMPT".to_string()))?,
            self.storyline
        );
        let unique_place_names: Vec<String> =
            self.scenes.scenes.iter().map(|s| s.place.clone()).collect();
        for place_name in unique_place_names {
            let place_suffix = format!("Place: {}\nDescription: ", place_name);
            let place_text = self
                .text_generator
                .generate_text(
                    &format!("{}{}", place_prefix, place_suffix),
                    client,
                    filter,
                    Some(self.hyperparameters.sample_length_place),
                    self.hyperparameters.max_paragraph_length,
                    seed,
                    num_samples,
                    Some(self.hyperparameters.max_num_repetitions),
                )
                .await?;
            let place = Place::from_string(&place_name, &format!("{}{}", place_suffix, place_text));
            place_descriptions.insert(place_name.clone(), place);
            place_prefixes.push(format!("{}{}", place_prefix, place_suffix));
        }
        Ok((place_descriptions, place_prefixes))
    }

    pub async fn generate_dialog(
        &self,
        scenes: &[Scene],
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(String, String), StoryGenerationError> {
        let scene = scenes.last().ok_or_else(|| {
            StoryGenerationError::Other(anyhow::anyhow!("No scenes provided"))
        })?;
        let mut place_t = format!("Place: {}\n", scene.place);
        if let Some(place_description) = self.places.get(&scene.place) {
            place_t.push_str(&format!("Description: {}\n", place_description.description));
        }
        let mut characters_t = String::from("Characters: ");
        for (name, description) in &self.characters.character_descriptions {
            if scene.beat.contains(name) {
                characters_t.push_str(&format!("{}: {}\n", name, description));
            }
        }
        let plot_element_t = format!("Plot element: {}\n", scene.plot_element);
        let summary_t = if scenes.len() > 1 {
            format!(
                "Summary: {}\nPrevious beat: {}\n",
                self.storyline,
                scenes[scenes.len() - 2].beat
            )
        } else {
            format!("Summary: {}\n", self.storyline)
        };
        let beat_t = format!("Beat: {}\n", scene.beat);
        let dialog_prefix = format!(
            "{}{}{}{}{}\n**Dialog:**\n",
            self.prefixes.get("DIALOG_PROMPT").ok_or_else(|| {
                StoryGenerationError::MissingPrompt("DIALOG_PROMPT".to_string())
            })?,
            place_t,
            characters_t,
            plot_element_t,
            summary_t,
            beat_t
        );
        let dialog = self
            .text_generator
            .generate_text(
                &dialog_prefix,
                client,
                filter,
                None,
                self.hyperparameters.max_paragraph_length,
                seed,
                num_samples,
                Some(self.hyperparameters.max_num_repetitions),
            )
            .await?;
        Ok((dialog, dialog_prefix))
    }

    pub async fn step(
        &mut self,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        level: Option<usize>,
        seed: Option<u64>,
        idx: Option<i32>,
    ) -> Result<bool, StoryGenerationError> {
        let level = level.unwrap_or(self.level);
        if level >= LEVEL_NAMES.len() {
            return Err(StoryGenerationError::InvalidLevel(level));
        }
        self.level = level + 1;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.interventions
            .insert(timestamp, format!("STEP {}\n", self.level));

        match self.level {
            1 => {
                let (title, titles_prefix) = self
                    .generate_title(client, filter, seed, self.hyperparameters.num_samples)
                    .await?;
                self.title = title.clone();
                self.prompts
                    .get_mut("title")
                    .unwrap()
                    .push(titles_prefix.clone());
                self.interventions
                    .get_mut(&timestamp)
                    .unwrap()
                    .push_str(&title.to_string());
                Ok(!title.title.is_empty())
            }
            2 => {
                let (characters, character_prompts) = self
                    .generate_characters(client, filter, seed, self.hyperparameters.num_samples)
                    .await?;
                self.characters = characters.clone();
                self.prompts
                    .get_mut("characters")
                    .unwrap()
                    .push(character_prompts.clone());
                self.interventions
                    .get_mut(&timestamp)
                    .unwrap()
                    .push_str(&characters.to_string());
                Ok(!characters.character_descriptions.is_empty())
            }
            3 => {
                let (scenes, scene_prompts) = self
                    .generate_scenes(client, filter, seed, self.hyperparameters.num_samples)
                    .await?;
                self.scenes = scenes.clone();
                self.prompts
                    .get_mut("scenes")
                    .unwrap()
                    .push(scene_prompts.clone());
                self.interventions
                    .get_mut(&timestamp)
                    .unwrap()
                    .push_str(&scenes.to_string());
                Ok(!scenes.scenes.is_empty())
            }
            4 => {
                let (place_descriptions, place_prompts) = self
                    .generate_place_descriptions(client, filter, seed, self.hyperparameters.num_samples)
                    .await?;
                self.places = place_descriptions.clone();
                self.prompts.get_mut("places").unwrap().extend(place_prompts);
                for place in place_descriptions.values() {
                    self.interventions
                        .get_mut(&timestamp)
                        .unwrap()
                        .push_str(&place.to_string());
                }
                let num_places = self
                    .scenes
                    .scenes
                    .iter()
                    .map(|s| s.place.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                Ok(place_descriptions.len() == num_places && num_places > 0)
            }
            5 => {
                let num_scenes = self.num_scenes();
                if let Some(idx) = idx {
                    if idx < 0 || idx as usize >= num_scenes {
                        return Err(StoryGenerationError::InvalidSceneIndex(idx));
                    }
                    while self.dialogs.len() < num_scenes {
                        self.dialogs.push(String::new());
                    }
                    while self.prompts.get("dialogs").unwrap().len() < num_scenes {
                        self.prompts.get_mut("dialogs").unwrap().push(String::new());
                    }
                    let (dialog, dialog_prompt) = self
                        .generate_dialog(
                            &self.scenes.scenes[..=idx as usize],
                            client,
                            filter,
                            seed,
                            self.hyperparameters.num_samples,
                        )
                        .await?;
                    self.dialogs[idx as usize] = dialog.clone();
                    self.prompts.get_mut("dialogs").unwrap()[idx as usize] = dialog_prompt.clone();
                    self.interventions
                        .get_mut(&timestamp)
                        .unwrap()
                        .push_str(&dialog);
                } else {
                    let mut dialogs = Vec::new();
                    let mut dialog_prompts = Vec::new();
                    for k in 0..num_scenes {
                        let (dialog, dialog_prompt) = self
                            .generate_dialog(
                                &self.scenes.scenes[..=k],
                                client,
                                filter,
                                seed,
                                self.hyperparameters.num_samples,
                            )
                            .await?;
                        dialogs.push(dialog.clone());
                        dialog_prompts.push(dialog_prompt.clone());
                        self.interventions
                            .get_mut(&timestamp)
                            .unwrap()
                            .push_str(&dialog);
                    }
                    self.dialogs = dialogs;
                    self.prompts.get_mut("dialogs").unwrap().extend(dialog_prompts);
                }
                Ok(true)
            }
            _ => Err(StoryGenerationError::InvalidLevel(self.level)),
        }
    }

    pub fn get_story(&self) -> Story {
        Story {
            storyline: self.storyline.clone(),
            title: self.title.title.clone(),
            character_descriptions: self.characters.character_descriptions.clone(),
            place_descriptions: self.places.clone(),
            scenes: self.scenes.clone(),
            dialogs: self.dialogs.clone(),
        }
    }

    pub fn rewrite(
        &mut self,
        text: String,
        level: usize,
        entity: Option<String>,
    ) -> Result<(), StoryGenerationError> {
        if level >= LEVEL_NAMES.len() {
            return Err(StoryGenerationError::InvalidLevel(level));
        }
        let mut prompt_diff = String::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        match level {
            0 => {
                prompt_diff = diff_prompt_change_str(&self.storyline, &text);
                self.set_storyline(text);
            }
            1 => {
                let title = Title::from_string(&text);
                prompt_diff = diff_prompt_change_str(&self.title.title, &title.title);
                self.title = title;
            }
            2 => {
                let characters = Characters::from_string(&text);
                prompt_diff = diff_prompt_change_dict(
                    &self.characters.character_descriptions,
                    &characters.character_descriptions,
                );
                self.characters = characters;
            }
            3 => {
                let scenes = Scenes::from_string(&text);
                prompt_diff = diff_prompt_change_scenes(&self.scenes.scenes, &scenes.scenes);
                self.scenes = scenes;
            }
            4 => {
                if let Some(entity) = entity {
                    if self.places.contains_key(&entity) {
                        let place_prefix = format!("Place: {}\nDescription: ", entity);
                        let place = Place::from_string(&entity, &format!("{}{}", place_prefix, text));
                        prompt_diff = diff_prompt_change_str(
                            &self.places.get(&entity).unwrap().name,
                            &place.name,
                        );
                        prompt_diff.push_str(&format!(
                            "\n{}",
                            diff_prompt_change_str(
                                &self.places.get(&entity).unwrap().description,
                                &place.description
                            )
                        ));
                        self.places.insert(entity.clone(), place);
                    } else {
                        return Err(StoryGenerationError::PlaceNotFound(entity));
                    }
                }
            }
            5 => {
                if let Some(entity) = entity {
                    let idx: usize = entity
                        .parse()
                        .map_err(|_| StoryGenerationError::InvalidSceneIndex(-1))?;
                    if idx < self.num_scenes() {
                        prompt_diff = diff_prompt_change_str(&self.dialogs[idx], &text);
                        self.dialogs[idx] = text;
                    } else {
                        return Err(StoryGenerationError::InvalidSceneIndex(idx as i32));
                    }
                }
            }
            _ => return Err(StoryGenerationError::InvalidLevel(level)),
        }

        if !prompt_diff.is_empty() {
            let intervention = format!(
                "REWRITE {}{}\n{}",
                LEVEL_NAMES[level],
                entity.map(|e| format!(" {}", e)).unwrap_or_default(),
                prompt_diff
            );
            self.interventions.insert(timestamp, intervention);
        }

        Ok(())
    }

    pub async fn complete(
        &mut self,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        level: usize,
        seed: Option<u64>,
        entity: Option<usize>,
        sample_length: usize,
    ) -> Result<(), StoryGenerationError> {
        if level >= LEVEL_NAMES.len() {
            return Err(StoryGenerationError::InvalidLevel(level));
        }
        let mut prompt_diff = String::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        match level {
            2 => {
                let text_characters = strip_remove_end(&self.characters.to_string());
                let prompt = format!(
                    "{}{}",
                    self.prompts
                        .get("characters")
                        .unwrap()
                        .last()
                        .unwrap(),
                    text_characters
                );
                let text = self
                    .text_generator
                    .generate_text(
                        &prompt,
                        client,
                        filter,
                        Some(sample_length),
                        sample_length,
                        seed,
                        1,
                        Some(self.hyperparameters.max_num_repetitions),
                    )
                    .await?;
                let new_characters = Characters::from_string(&format!("{}{}", text_characters, text));
                prompt_diff = diff_prompt_change_dict(
                    &self.characters.character_descriptions,
                    &new_characters.character_descriptions,
                );
                self.characters = new_characters;
            }
            3 => {
                let text_scenes = strip_remove_end(&self.scenes.to_string());
                let prompt = format!(
                    "{}{}",
                    self.prompts.get("scenes").unwrap().last().unwrap(),
                    text_scenes
                );
                let text = self
                    .text_generator
                    .generate_text(
                        &prompt,
                        client,
                        filter,
                        Some(sample_length),
                        sample_length,
                        seed,
                        1,
                        Some(self.hyperparameters.max_num_repetitions),
                    )
                    .await?;
                let new_scenes = Scenes::from_string(&format!("{}{}", text_scenes, text));
                prompt_diff = diff_prompt_change_scenes(&self.scenes.scenes, &new_scenes.scenes);
                self.scenes = new_scenes;
            }
            5 => {
                let num_scenes = self.num_scenes();
                while self.dialogs.len() < num_scenes {
                    self.dialogs.push(String::new());
                }
                while self.prompts.get("dialogs").unwrap().len() < num_scenes {
                    self.prompts.get_mut("dialogs").unwrap().push(String::new());
                }
                if let Some(idx) = entity {
                    if idx < num_scenes {
                        let prompt = format!(
                            "{}{}",
                            self.prompts.get("dialogs").unwrap()[idx],
                            self.dialogs[idx]
                        );
                        let text = self
                            .text_generator
                            .generate_text(
                                &prompt,
                                client,
                                filter,
                                Some(sample_length),
                                sample_length,
                                seed,
                                1,
                                Some(self.hyperparameters.max_num_repetitions),
                            )
                            .await?;
                        let new_dialog = format!("{}{}", self.dialogs[idx], text);
                        prompt_diff = diff_prompt_change_str(&self.dialogs[idx], &new_dialog);
                        self.dialogs[idx] = new_dialog;
                    } else {
                        return Err(StoryGenerationError::InvalidSceneIndex(idx as i32));
                    }
                }
            }
            _ => return Err(StoryGenerationError::InvalidLevel(level)),
        }

        if !prompt_diff.is_empty() {
            let intervention = format!(
                "COMPLETE {}{}\n{}",
                LEVEL_NAMES[level],
                entity.map(|e| format!(" {}", e)).unwrap_or_default(),
                prompt_diff
            );
            self.interventions.insert(timestamp, intervention);
        }

        Ok(())
    }
}