use async_trait::async_trait;

use crate::config::hyperparameters::Hyperparameters;

#[derive(Debug, Clone, PartialEq)]
pub struct LanguageResponse {
    pub prompt: String,
    pub prompt_length: usize,
    pub text: String,
    pub text_length: usize,
}


#[derive(Debug)]
pub enum LanguageApiEnum {
    Custom(CustomLanguageApi),
}

impl LanguageApiEnum {
    pub async fn sample(
        &self,
        prompt: &str,
        sample_length: Option<usize>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> anyhow::Result<Vec<LanguageResponse>> {
        match self {
            LanguageApiEnum::Custom(api) => api.sample(prompt, sample_length, seed, num_samples).await,
        }
    }
}

#[async_trait]
pub trait LanguageApi: Send + Sync {
    async fn sample(
        &self,
        prompt: &str,
        sample_length: Option<usize>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> anyhow::Result<Vec<LanguageResponse>>;
}

#[derive(Debug)]
pub struct CustomLanguageApi {
    pub hyperparameters: Hyperparameters,
    pub model: Option<String>,
    pub model_param: Option<String>,
    pub config_sampling: Option<serde_json::Value>,
}

impl CustomLanguageApi {
    pub fn new(
        hyperparameters: Hyperparameters,
        model: Option<String>,
        model_param: Option<String>,
        config_sampling: Option<serde_json::Value>,
    ) -> Self {
        Self {
            hyperparameters,
            model,
            model_param,
            config_sampling,
        }
    }
}

#[async_trait]
impl LanguageApi for CustomLanguageApi {
    async fn sample(
        &self,
        prompt: &str,
        sample_length: Option<usize>,
        _seed: Option<u64>,
        num_samples: usize,
    ) -> anyhow::Result<Vec<LanguageResponse>> {
        let sample_length = sample_length.unwrap_or(self.hyperparameters.sample_length);
        let mut responses = Vec::new();
        for _ in 0..num_samples {
            // Mock response: echo prompt with a suffix
            let text = format!("Generated text for prompt: {} **END**", prompt);
            let text = text.chars().take(sample_length).collect::<String>();
            responses.push(LanguageResponse {
                prompt: prompt.to_string(),
                prompt_length: prompt.len(),
                text: text.clone(),
                text_length: text.len(),
            });
        }
        Ok(responses)
    }
}