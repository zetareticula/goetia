use std::collections::HashMap;

use crate::api::filter_api::{FilterApi, FilterApiEnum};
use crate::api::language_api::LanguageApiEnum;
use crate::models::Scene;
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
    InvalidSceneIndex(usize),
    #[error("Missing prompt: {0}")]
    MissingPrompt(String),
}

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
    interventions: HashMap<u64, String>,
    level: usize,
    level_names: &'static [&'static str],
}

impl StoryGenerator {
    pub fn new(
        storyline: String,
        prefixes: PromptTemplates,
        hyperparameters: Hyperparameters,
        text_generator: TextGenerator,
    ) -> Self {
        let mut generator = Self {
            storyline: storyline.clone(),
            prefixes,
            hyperparameters: hyperparameters.clone(),
            text_generator,
            prompts: HashMap::from([
                ("title".to_string(), vec![]),
                ("characters".to_string(), vec![]),
                ("scenes".to_string(), vec![]),
                ("places".to_string(), vec![]),
                ("dialogs".to_string(), vec![]),
            ]),
            title: Title::new(String::new()),
            characters: Characters {
                character_descriptions: HashMap::new(),
            },
            scenes: Scenes::new(),
            places: HashMap::new(),
            dialogs: Vec::new(),
            interventions: HashMap::new(),
            level: 0,
            level_names: &["storyline", "title", "characters", "scenes", "places", "dialogs"],
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
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;
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

    pub fn dialogs(&self) -> &Vec<String> {
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
        client: &LanguageApiEnum,
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
        client: &LanguageApiEnum,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(Characters, String), StoryGenerationError> {
        let characters_prompt = self.prefixes
            .get("CHARACTERS_PROMPT")
            .ok_or_else(|| StoryGenerationError::MissingPrompt("CHARACTERS_PROMPT".to_string()))?
            .to_string();
        let characters_prefix = format!(
            "{}{}",
            characters_prompt,
            self.storyline
        );
        let characters_text = self
            .text_generator
            .generate_text(
                &characters_prefix,
                client,
                filter.as_ref(),
                Some(self.hyperparameters.sample_length),
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
        character_descriptions: &HashMap<String, String>,
        client: &LanguageApiEnum,
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
        for description in character_descriptions.values() {
            scenes_prefix.push_str(&format!("{}\n", description));
        }
        scenes_prefix.push_str("\n**Scenes:**");
        let scenes_text = self
            .text_generator
            .generate_text(
                &scenes_prefix,
                client,
                filter.as_ref(),
                Some(self.hyperparameters.sample_length),
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
        scenes: &Scenes,
        client: &LanguageApiEnum,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(HashMap<String, Place>, Vec<String>), StoryGenerationError> {
        let mut place_descriptions = HashMap::new();
        let mut place_prefixes = Vec::new();
        let unique_place_names: std::collections::HashSet<String> =
            scenes.scenes.iter().map(|scene| scene.place.clone()).collect();
        let place_prefix_base = format!(
            "{}{}\n",
            self.prefixes
                .get("SETTING_PROMPT")
                .ok_or_else(|| StoryGenerationError::MissingPrompt("SETTING_PROMPT".to_string()))?,
            self.storyline
        );

        for place_name in unique_place_names {
            let place_suffix = format!("Place: {}\nDescription:", place_name);
            let place_text = self
                .text_generator
                .generate_text(
                    &format!("{}{}", place_prefix_base, place_suffix),
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
            place_prefixes.push(format!("{}{}", place_prefix_base, place_suffix));
        }
        Ok((place_descriptions, place_prefixes))
    }

    pub async fn generate_dialog(
        &self,
        scenes: &[Scene],
        character_descriptions: &HashMap<String, String>,
        place_descriptions: &HashMap<String, Place>,
        client: &LanguageApiEnum,
        filter: Option<&dyn FilterApi>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<(String, String), StoryGenerationError> {
        let scene = scenes.last().ok_or_else(|| {
            StoryGenerationError::InvalidSceneIndex(0)
        })?;
        let mut place_t = format!("Place: {}\n", scene.place);
        if let Some(place_description) = place_descriptions.get(&scene.place) {
            place_t.push_str(&format!("Description: {}\n", place_description.description));
        }
        let mut characters_t = String::from("Characters: ");
        for (name, description) in character_descriptions {
            if scene.beat.contains(name) {
                characters_t.push_str(&format!("{}\n", description));
            }
        }
        let plot_element_t = format!("Plot element: {}\n", scene.plot_element);
        let summary_t = format!(
            "Summary: {}\n",
            if scenes.len() > 1 {
                format!("Previous beat: {}\n", scenes[scenes.len() - 2].beat)
            } else {
                String::new()
            }
        );
        let beat_t = format!("Beat: {}\n", scene.beat);
        let dialog_prefix = format!(
            "**Dialog:**\n"
        );
        let dialog = self
            .text_generator
            .generate_text(
                &dialog_prefix,
                client,
                filter.as_ref(),
                Some(self.hyperparameters.sample_length),
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
        level: Option<usize>,
        seed: Option<u64>,
        idx: Option<usize>,
        client: &LanguageApiEnum,
        filter: Option<&dyn FilterApi>,
    ) -> Result<bool, StoryGenerationError> {
        let level = level.unwrap_or(self.level);
        if level >= self.level_names.len() {
            return Err(StoryGenerationError::InvalidLevel(level));
        }
        self.level = level + 1;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.interventions
            .insert(timestamp as u64, format!("STEP {}\n", self.level));

        match self.level {
            1 => {
                let (title, titles_prefix) = self.generate_title(client, filter, seed, self.hyperparameters.num_samples).await?;
                self.interventions
                    .entry(timestamp as u64)
                    .and_modify(|e| *e += &title.to_string());
                self.prompts.get_mut("title").unwrap().push(titles_prefix);
                self.title = title;
                Ok(!self.title.title.is_empty())
            }
            2 => {
                let (characters, character_prompts) = self.generate_characters(client, filter, seed, self.hyperparameters.num_samples).await?;
                self.interventions
                    .entry(timestamp as u64)
                    .and_modify(|e| *e += &characters.to_string());
                self.prompts.get_mut("characters").unwrap().push(character_prompts);
                self.characters = characters;
                Ok(!self.characters.character_descriptions.is_empty())
            }
            3 => {
                let (scenes, scene_prompts) = self.generate_scenes(&self.characters.character_descriptions, client, filter, seed, self.hyperparameters.num_samples).await?;
                self.interventions
                    .entry(timestamp as u64)
                    .and_modify(|e| *e += &scenes.to_string());
                self.prompts.get_mut("scenes").unwrap().push(scene_prompts);
                self.scenes = scenes;
                Ok(!self.scenes.scenes.is_empty())
            }
            4 => {
                let (place_descriptions, place_prompts) = self.generate_place_descriptions(&self.scenes, client, filter, seed, self.hyperparameters.num_samples).await?;
                for place in place_descriptions.values() {
                    self.interventions
                        .entry(timestamp as u64)
                        .or_insert_with(|| format!("CHARACTER {}\n", idx.unwrap_or(0)));
                }
                self.prompts.get_mut("places").unwrap().extend(place_prompts);
                self.places = place_descriptions;
                let num_places = self.scenes.scenes.iter().map(|scene| scene.place.clone()).collect::<std::collections::HashSet<_>>().len();
                Ok(self.places.len() == num_places && num_places > 0)
            }
            5 => {
                if let Some(idx) = idx {
                    while self.dialogs.len() <= idx {
                        self.dialogs.push(String::new());
                        self.prompts.get_mut("dialogs").unwrap().push(String::new());
                    }
                    if idx >= self.scenes.scenes.len() {
                        return Err(StoryGenerationError::InvalidSceneIndex(idx));
                    }
                    let (dialog, dialog_prefix) = self.generate_dialog(
                        &self.scenes.scenes[..=idx],
                        &self.characters.character_descriptions,
                        &self.places,
                        client,
                        filter,
                        seed,
                        self.hyperparameters.num_samples,
                    ).await?;
                    self.dialogs[idx] = dialog.clone();
                    self.prompts.get_mut("dialogs").unwrap()[idx] = dialog_prefix;
                    self.interventions
                        .entry(timestamp as u64)
                        .or_insert_with(|| format!("STEP {}\n", level));
                } else {
                    let results: Vec<_> = (0..self.scenes.scenes.len())
                        .map(|k| async move {
                            self.generate_dialog(
                                &self.scenes.scenes[..=k],
                                &self.characters.character_descriptions,
                                &self.places,
                                client,
                                filter,
                                seed,
                                self.hyperparameters.num_samples,
                            )
                            .await
                        })
                        .collect::<Vec<_>>();
                    let results = futures::future::try_join_all(results).await?;
                    let (dialogs, dialog_prompts): (Vec<_>, Vec<_>) = results.into_iter().unzip();
                    for dialog in &dialogs {
                        self.interventions
                            .entry(timestamp as u64)
                            .and_modify(|e| *e += dialog);
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

    pub fn rewrite(&mut self, text: &str, level: usize, entity: Option<usize>) -> Result<(), StoryGenerationError> {
        if level >= self.level_names.len() {
            return Err(StoryGenerationError::InvalidLevel(level));
        }
        let mut prompt_diff = String::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        match level {
            0 => {
                prompt_diff = diff_prompt_change_str(&self.storyline, text);
                self.set_storyline(text.to_string());
            }
            1 => {
                let title = Title::from_string(text);
                prompt_diff = diff_prompt_change_str(&self.title.title, &title.title);
                self.title = title;
            }
            2 => {
                let characters = Characters::from_string(text);
                prompt_diff = diff_prompt_change_dict(
                    &self.characters.character_descriptions,
                    &characters.character_descriptions,
                );
                self.characters = characters;
            }
            3 => {
                let scenes = Scenes::from_string(text);
                prompt_diff = diff_prompt_change_scenes(&self.scenes.scenes, &scenes.scenes);
                self.scenes = scenes;
            }
            4 => {
                if let Some(entity) = entity {
                    let entity_str = entity.to_string();
                    if self.places.contains_key(&entity_str) {
                        let place_prefix = format!("Place: {}\nDescription:", entity_str);
                        let place = Place::from_string(&entity_str, &format!("{}{}", place_prefix, text));
                        prompt_diff = diff_prompt_change_str(&self.places[&entity_str].name, &place.name);
                        prompt_diff.push_str(&format!(
                            "\n{}",
                            diff_prompt_change_str(&self.places[&entity_str].description, &place.description)
                        ));
                        self.places.insert(entity_str, place);
                    }
                }
            }
            5 => {
                if let Some(idx) = entity {
                    if idx >= self.scenes.scenes.len() {
                        return Err(StoryGenerationError::InvalidSceneIndex(idx));
                    }
                    prompt_diff = diff_prompt_change_str(&self.dialogs[idx], text);
                    self.dialogs[idx] = text.to_string();
                }
            }
            _ => return Err(StoryGenerationError::InvalidLevel(level)),
        }

        if !prompt_diff.is_empty() {
            let intervention = format!(
                "REWRITE {} {}\n{}",
                self.level_names[level],
                entity.map_or(String::new(), |e| e.to_string()),
                prompt_diff
            );
            self.interventions.insert(timestamp as u64, intervention);
        }
        Ok(())
    }

    pub async fn complete(
        &mut self,
        level: usize,
        seed: Option<u64>,
        entity: Option<usize>,
        client: &LanguageApiEnum,
        filter: Option<FilterApiEnum>,
    ) -> Result<(), StoryGenerationError> {
        if level >= self.level_names.len() {
            return Err(StoryGenerationError::InvalidLevel(level));
        }
        let mut prompt_diff = String::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        match level {
            2 => {
                let text_characters = strip_remove_end(&self.characters.to_string());
                let prompt = format!(
                    "{}{}",
                    self.prompts.get("characters").unwrap().last().unwrap_or(&String::new()),
                    text_characters
                );
                let text = self
                    .text_generator
                    .generate_text(
                        &prompt,
                        client,
                        filter.as_ref(),
                        Some(self.hyperparameters.sample_length),
                        self.hyperparameters.sample_length,
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
                    self.prompts.get("scenes").unwrap().last().unwrap_or(&String::new()),
                    text_scenes
                );
                let text = self
                    .text_generator
                    .generate_text(
                        &prompt,
                        client,
                        filter.as_ref(),
                        Some(self.hyperparameters.sample_length),
                        self.hyperparameters.sample_length,
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
                if let Some(idx) = entity {
                    while self.dialogs.len() <= idx {
                        self.dialogs.push(String::new());
                        self.prompts.get_mut("dialogs").unwrap().push(String::new());
                    }
                    if idx >= self.scenes.scenes.len() {
                        return Err(StoryGenerationError::InvalidSceneIndex(idx));
                    }
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
                            Some(self.hyperparameters.sample_length),
                            self.hyperparameters.sample_length,
                            seed,
                            1,
                            Some(self.hyperparameters.max_num_repetitions),
                        )
                        .await?;
                    let new_dialog = format!("{}{}", self.dialogs[idx], text);
                    prompt_diff = diff_prompt_change_str(&self.dialogs[idx], &new_dialog);
                    self.dialogs[idx] = new_dialog;
                }
            }
            _ => return Err(StoryGenerationError::InvalidLevel(level)),
        }

        if !prompt_diff.is_empty() {
            let intervention = format!(
                "COMPLETE {} {}\n{}",
                self.level_names[level],
                entity.map_or(String::new(), |e| e.to_string()),
                prompt_diff
            );
            self.interventions.insert(timestamp as u64, intervention);
        }
        Ok(())
    }
}