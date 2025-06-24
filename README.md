# Dramatron-rs

A Rust implementation of the Dramatron script-writing tool, designed for hierarchical story generation from a provided storyline. This project translates the functionality of the original Python-based Dramatron notebook into a modular, type-safe, and extensible Rust library, enabling script generation with customizable language models and content moderation.

## Features

- **Hierarchical Story Generation**: Generates scripts in stages (storyline, title, characters, scenes, places, dialogs) using a structured, level-based approach.
- **Modular Design**: Organized into modules for models, generation logic, API abstractions, prompts, and utilities, ensuring maintainability and scalability.
- **Trait-Based API Abstractions**: Supports pluggable language models and content filters via the `LanguageApi` and `FilterApi` traits, with mock implementations for testing.
- **Generation History**: Tracks story edits (`NEW`, `CONTINUE`, `REWRITE`) with navigation support, mirroring the Python implementation.
- **Type-Safe and Async**: Leverages Rust's type system and async/await for robust error handling and efficient API interactions.
- **Configurable Hyperparameters**: Customizable generation parameters (e.g., sampling probability, text length) via a `Hyperparameters` struct.
- **Extensible Prompts**: Prompt templates for generation steps, with support for customization or file-based loading.

## Project Structure

```
dramatron-rs/
├── Cargo.toml                 # Project metadata and dependencies
├── src/
│   ├── main.rs                # Entry point with example usage
│   ├── lib.rs                 # Library module declarations
│   ├── models/                # Data structures (Title, Character, Scene, etc.)
│   ├── generator/             # Story generation and history logic
│   ├── api/                   # Language and filter API abstractions
│   ├── utils/                 # Utility functions (text extraction, diff)
│   ├── prompts/               # Prompt templates and handling
│   ├── config/                # Configuration and hyperparameters
│   └── tests/                 # Unit and integration tests
├── examples/                  # Example scripts demonstrating usage
├── README.md                  # Project documentation
└── rustfmt.toml               # Code formatting configuration
```

## Installation

### Prerequisites

- Rust (stable, edition 2021 or later)
- Cargo (included with Rust)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/dramatron-rs.git
   cd dramatron-rs
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the example:
   ```bash
   cargo run --example simple_script
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Usage

The `dramatron-rs` library can be used to generate scripts programmatically. Below is an example of generating a story title using a mock language model:

```rust
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
```

### Custom Language Model

To use a real language model, implement the `LanguageApi` trait:

```rust
use async_trait::async_trait;
use dramatron_rs::api::{LanguageApi, LanguageResponse};

struct MyLanguageApi;

#[async_trait]
impl LanguageApi for MyLanguageApi {
    async fn sample(
        &self,
        prompt: &str,
        sample_length: Option<usize>,
        seed: Option<u64>,
        num_samples: usize,
    ) -> anyhow::Result<Vec<LanguageResponse>> {
        // Implement API call to your language model
        Ok(vec![])
    }
}
```

### Content Moderation

To use a real content moderation service (e.g., Perspective API), implement the `FilterApi` trait:

```rust
use async_trait::async_trait;
use dramatron_rs::api::FilterApi;

struct MyFilterApi;

#[async_trait]
impl FilterApi for MyFilterApi {
    async fn validate(&self, text: &str) -> bool {
        // Implement content moderation logic
        true
    }
}
```

## Configuration

Hyperparameters can be customized via the `Hyperparameters` struct. Example:

```rust
let mut hyperparameters = Hyperparameters::default();
hyperparameters.sampling_prob = 0.9;
hyperparameters.sample_length = 1024;
```

Prompt templates are defined in `PromptTemplates` and can be extended or loaded from a file for customization.

## Dependencies

- `serde`: Serialization/deserialization
- `rand`: Randomization for mock APIs
- `regex`: Text parsing
- `thiserror`: Custom error handling
- `async-trait`: Async trait support
- `diff`: Text diffing for rewrite/complete operations
- `reqwest` (optional): HTTP requests for real Perspective API (behind `http` feature)

## Limitations

- **Mock APIs**: The provided `CustomLanguageApi` and `PerspectiveApi` are mock implementations for testing. Users must implement these traits for real-world use.
- **Prompts**: Default prompts are inferred placeholders. Customization or file-based loading is recommended for production.
- **UI**: The original Python UI is not implemented. A CLI or GUI can be added using crates like `clap` or `eframe`.
- **Performance**: The mock APIs are synchronous for simplicity. Real implementations should optimize for asynchronous performance.

## Extensibility

- **Custom Prompts**: Extend `PromptTemplates` to load prompts from files (e.g., TOML, JSON).
- **Real APIs**: Implement `LanguageApi` and `FilterApi` for integration with external services like Hugging Face, OpenAI, or Perspective API.
- **UI**: Add a command-line interface with `clap` or a graphical interface with `eframe`.
- **Additional Features**: Support for script rendering in Fountain format, advanced diffing, or multi-language generation.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/my-feature`).
3. Commit changes (`git commit -am "Add my feature"`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a pull request.

Ensure code adheres to `rustfmt` (run `cargo fmt`) and passes tests (`cargo test`).

## License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE) for details.

## Acknowledgments

Inspired by the original Dramatron Python notebook, this project aims to provide a robust, production-ready Rust alternative for script generation.

---

