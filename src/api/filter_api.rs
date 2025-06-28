use async_trait::async_trait;
use std::fmt::Debug;
use std::fmt::Write;






#[derive(Debug)]
pub enum FilterApiEnum {
    Mock(MockFilterApi),
    Perspective(PerspectiveFilterApi),
}

impl FilterApiEnum {
    pub fn filter(&self, text: &str) -> bool {
        match self {
            FilterApiEnum::Mock(api) => api.filter(text),
            FilterApiEnum::Perspective(api) => api.filter(text),
        }
    }
}

pub trait FilterApi: Send + Sync {
    fn filter(&self, text: &str) -> bool;
}

pub struct MockFilterApi;

impl Debug for MockFilterApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockFilterApi")
    }
}

impl FilterApi for MockFilterApi {
    fn filter(&self, text: &str) -> bool {
        true
    }
}

pub struct PerspectiveFilterApi;

impl Debug for PerspectiveFilterApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PerspectiveFilterApi")
    }
}

impl FilterApi for PerspectiveFilterApi {
    fn filter(&self, text: &str) -> bool {
        // Mock implementation: check for simple keywords
        let toxic_words = vec!["hate", "insult", "offensive"];
        !toxic_words.iter().any(|word| text.to_lowercase().contains(word))
    }
}