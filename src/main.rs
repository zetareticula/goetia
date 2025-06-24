use dramatron_rs::api::{CustomLanguageApi, MockFilterApi};
use dramatron_rs::config::hyperparameters::Hyperparameters;
use dramatron_rs::generator::story_generator::StoryGenerator;
use dramatron_rs::generator::text_generator::TextGenerator;
use dramatron_rs::prompts::templates::PromptTemplates;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hyperparameters = Hyperparameters::default();
    let text_generator = TextGenerator::new(hyperparameters.clone());
    let prefixes = PromptTemplates::new();
    let mut generator = StoryGenerator::new(
        "A hero saves the world from an alien invasion.".to_string(),
        prefixes,
        hyperparameters.clone(),
        text_generator,
    );

    let language_api = CustomLanguageApi::new(hyperparameters, None, None, None);
    let filter_api = MockFilterApi;

    let success = generator
        .step(None, None, None, &language_api, Some(&filter_api))
        .await?;
    println!("Step successful: {}", success);
    println!("Generated Title: {}", generator.title_str());

    Ok(())
}