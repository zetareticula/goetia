#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation_history() {
        let mut history = GenerationHistory::new();
        history.add("First".to_string(), GenerationAction::New);
        assert_eq!(history.previous(), Some(&"First".to_string()));
        history.add("Second".to_string(), GenerationAction::Continue);
        assert_eq!(history.next(), Some(&"Second".to_string()));
        history.add("Third".to_string(), GenerationAction::Rewrite);
        assert_eq!(history.previous(), Some(&"First".to_string()));
        history.add("Fourth".to_string(), GenerationAction::Rewrite);
        assert_eq!(history.next(), Some(&"Fourth".to_string()));
    }
}