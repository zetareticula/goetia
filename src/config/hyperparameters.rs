#[derive(Debug, Clone)]
pub struct Hyperparameters {
    pub default_seed: u64,
    pub sampling_prob: f64,
    pub sampling_temp: f64,
    pub sample_length_title: usize,
    pub sample_length_place: usize,
    pub sample_length: usize,
    pub max_paragraph_length_characters: usize,
    pub max_paragraph_length_scenes: usize,
    pub max_paragraph_length: usize,
    pub max_retries: usize,
    pub max_num_repetitions: usize,
    pub max_num_attempts_get_out_of_loop: usize,
    pub num_samples: usize,
}

impl Default for Hyperparameters {
    fn default() -> Self {
        Self {
            default_seed: 1,
            sampling_prob: 0.95,
            sampling_temp: 1.0,
            sample_length_title: 64,
            sample_length_place: 128,
            sample_length: 511,
            max_paragraph_length_characters: 1024,
            max_paragraph_length_scenes: 1024,
            max_paragraph_length: 1024,
            max_retries: 10,
            max_num_repetitions: 3,
            max_num_attempts_get_out_of_loop: 3,
            num_samples: 1,
        }
    }
}