use dramatron_rs::api::{MockFilterApi, MockLanguageApi};
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
        hyperparameters,
        text_generator,
    );

    let language_api = MockLanguageApi::new(hyperparameters);
    let filter_api = MockFilterApi;

    let (title, _) = generator
        .generate_title(&language_api, Some(&filter_api), None, 1)
        .await?;
    println!("Generated Title: {}", title);

    Ok(())
}