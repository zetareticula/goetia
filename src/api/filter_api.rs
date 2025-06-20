use async_trait::async_trait;

#[async_trait]
pub trait FilterApi: Send + Sync {
    async fn validate(&self, text: &str) -> bool;
}

pub struct MockFilterApi;

#[async_trait]
impl FilterApi for MockFilterApi {
    async fn validate(&self, _text: &str) -> bool {
        true // Always pass for mock
    }
}

pub struct PerspectiveApi {
    key: String,
    thresholds: std::collections::HashMap<String, f64>,
}

impl PerspectiveApi {
    pub fn new(key: String, thresholds: std::collections::HashMap<String, f64>) -> Self {
        Self { key, thresholds }
    }
}

#[async_trait]
impl FilterApi for PerspectiveApi {
    async fn validate(&self, text: &str) -> bool {
        // Mock implementation: check for simple keywords
        let toxic_words = vec!["hate", "insult", "offensive"];
        !toxic_words.iter().any(|word| text.to_lowercase().contains(word))
    }
}