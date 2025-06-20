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