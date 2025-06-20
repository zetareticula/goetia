use crate::api::{FilterApi, LanguageApi, LanguageResponse};
use crate::config::hyperparameters::Hyperparameters;

#[derive(Debug, thiserror::Error)]
pub enum TextGenerationError {
    #[error("Content filtered out")]
    Filtered,
    #[error("Loop detected")]
    LoopDetected,
    #[error("API error: {0}")]
    ApiError(#[from] anyhow::Error),
}

pub struct TextGenerator {
    hyperparameters: Hyperparameters,
}

impl TextGenerator {
    pub fn new(hyperparameters: Hyperparameters) -> Self {
        Self { hyperparameters }
    }

    pub async fn generate_text(
        &self,
        generation_prompt: &str,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        sample_length: Option<usize>,
        max_paragraph_length: usize,
        seed: Option<u64>,
        num_samples: usize,
        max_num_repetitions: Option<usize>,
    ) -> Result<String, TextGenerationError> {
        let sample_length = sample_length.unwrap_or(self.hyperparameters.sample_length);
        let max_num_calls = max_paragraph_length / sample_length + 1;
        let mut num_calls = 0;
        let mut result = String::new();

        while num_calls < max_num_calls {
            let prompt = format!("{}{}", generation_prompt, result);
            let mut current_seed = seed.unwrap_or(self.hyperparameters.default_seed);
            let mut success = false;
            let mut attempts = 0;

            while !success && attempts < self.hyperparameters.max_num_attempts_get_out_of_loop {
                let responses = client
                    .sample(&prompt, Some(sample_length), Some(current_seed), num_samples)
                    .await?;
                let response = responses.first().ok_or_else(|| {
                    anyhow::anyhow!("No response from language API")
                })?;

                if let Some(filter) = filter {
                    if !filter.validate(&response.text).await {
                        return Ok(format!("Content was filtered out. **END**"));
                    }
                }

                if let Some(max_reps) = max_num_repetitions {
                    if self.detect_loop(&response.text, max_reps) {
                        current_seed += 1;
                        attempts += 1;
                        continue;
                    }
                }

                success = true;
                result.push_str(&response.text);
            }

            num_calls += 1;

            if result.contains("**END**") {
                let index = result.find("**END**").unwrap();
                return Ok(result[..index + "**END**".len()].to_string());
            }

            if result.contains("Example ") {
                let index = result.find("Example ").unwrap();
                return Ok(format!("{}{}", &result[..index], "**END**"));
            }

            if result.len() > max_paragraph_length {
                return Ok(format!("{}{}", result, "**END**"));
            }
        }

        Ok(format!("{}{}", result, "**END**"))
    }

    fn detect_loop(&self, text: &str, max_num_repetitions: usize) -> bool {
        let blocks: Vec<&str> = text.split("\n\n").collect();
        let mut block_counts = std::collections::HashMap::new();
        for block in blocks {
            *block_counts.entry(block).or_insert(0) += 1;
            if block_counts[block] > max_num_repetitions {
                return true;
            }
        }
        false
    }

    pub async fn generate_text_no_loop(
        &self,
        generation_prompt: &str,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        sample_length: Option<usize>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<String, TextGenerationError> {
        self.generate_text(
            generation_prompt,
            client,
            filter,
            sample_length,
            sample_length.unwrap_or(self.hyperparameters.sample_length),
            seed,
            num_samples,
            None,
        )
        .await
    }
}use crate::api::{FilterApi, LanguageApi, LanguageResponse};
use crate::config::hyperparameters::Hyperparameters;

#[derive(Debug, thiserror::Error)]
pub enum TextGenerationError {
    #[error("Content filtered out")]
    Filtered,
    #[error("Loop detected")]
    LoopDetected,
    #[error("API error: {0}")]
    ApiError(#[from] anyhow::Error),
}

pub struct TextGenerator {
    hyperparameters: Hyperparameters,
}

impl TextGenerator {
    pub fn new(hyperparameters: Hyperparameters) -> Self {
        Self { hyperparameters }
    }

    pub async fn generate_text(
        &self,
        generation_prompt: &str,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        sample_length: Option<usize>,
        max_paragraph_length: usize,
        seed: Option<u64>,
        num_samples: usize,
        max_num_repetitions: Option<usize>,
    ) -> Result<String, TextGenerationError> {
        let sample_length = sample_length.unwrap_or(self.hyperparameters.sample_length);
        let max_num_calls = max_paragraph_length / sample_length + 1;
        let mut num_calls = 0;
        let mut result = String::new();

        while num_calls < max_num_calls {
            let prompt = format!("{}{}", generation_prompt, result);
            let mut current_seed = seed.unwrap_or(self.hyperparameters.default_seed);
            let mut success = false;
            let mut attempts = 0;

            while !success && attempts < self.hyperparameters.max_num_attempts_get_out_of_loop {
                let responses = client
                    .sample(&prompt, Some(sample_length), Some(current_seed), num_samples)
                    .await?;
                let response = responses.first().ok_or_else(|| {
                    anyhow::anyhow!("No response from language API")
                })?;

                if let Some(filter) = filter {
                    if !filter.validate(&response.text).await {
                        return Ok(format!("Content was filtered out. **END**"));
                    }
                }

                if let Some(max_reps) = max_num_repetitions {
                    if self.detect_loop(&response.text, max_reps) {
                        current_seed += 1;
                        attempts += 1;
                        continue;
                    }
                }

                success = true;
                result.push_str(&response.text);
            }

            num_calls += 1;

            if result.contains("**END**") {
                let index = result.find("**END**").unwrap();
                return Ok(result[..index + "**END**".len()].to_string());
            }

            if result.contains("Example ") {
                let index = result.find("Example ").unwrap();
                return Ok(format!("{}{}", &result[..index], "**END**"));
            }

            if result.len() > max_paragraph_length {
                return Ok(format!("{}{}", result, "**END**"));
            }
        }

        Ok(format!("{}{}", result, "**END**"))
    }

    fn detect_loop(&self, text: &str, max_num_repetitions: usize) -> bool {
        let blocks: Vec<&str> = text.split("\n\n").collect();
        let mut block_counts = std::collections::HashMap::new();
        for block in blocks {
            *block_counts.entry(block).or_insert(0) += 1;
            if block_counts[block] > max_num_repetitions {
                return true;
            }
        }
        false
    }

    pub async fn generate_text_no_loop(
        &self,
        generation_prompt: &str,
        client: &dyn LanguageApi,
        filter: Option<&dyn FilterApi>,
        sample_length: Option<usize>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> Result<String, TextGenerationError> {
        self.generate_text(
            generation_prompt,
            client,
            filter,
            sample_length,
            sample_length.unwrap_or(self.hyperparameters.sample_length),
            seed,
            num_samples,
            None,
        )
        .await
    }
}