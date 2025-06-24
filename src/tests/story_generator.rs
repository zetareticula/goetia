#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{CustomLanguageApi, MockFilterApi};
    use tokio::test;

    #[test]
    async fn test_generate_title() {
        let hyperparameters = Hyperparameters::default();
        let text_generator = TextGenerator::new(hyperparameters.clone());
        let prefixes = PromptTemplates::new();
        let mut generator = StoryGenerator::new(
            "Test storyline.".to_string(),
            prefixes,
            hyperparameters.clone(),
            text_generator,
        );
        let language_api = CustomLanguageApi::new(hyperparameters, None, None, None);
        let filter_api = MockFilterApi;

        let (title, _) = generator
            .generate_title(&language_api, Some(&filter_api), None, 1)
            .await
            .unwrap();
        assert!(!title.title.is_empty());
    }

    #[test]
    async fn test_step() {
        let hyperparameters = Hyperparameters::default();
        let text_generator = TextGenerator::new(hyperparameters.clone());
        let prefixes = PromptTemplates::new();
        let mut generator = StoryGenerator::new(
            "Test storyline.".to_string(),
            prefixes,
            hyperparameters.clone(),
            text_generator,
        );
        let language_api = CustomLanguageApi::new(hyperparameters, None, None, None);
        let filter_api = MockFilterApi;

        let success = generator
            .step(None, None, None, &language_api, Some(&filter_api))
            .await
            .unwrap();
        assert!(success);
        assert_eq!(generator.level, 1);
    }
}