#[derive(Debug, Clone, PartialEq)]
pub enum GenerationAction {
    New,
    Continue,
    Rewrite,
}

#[derive(Debug, Clone)]
pub struct GenerationHistory<T> {
    items: Vec<T>,
    actions: Vec<GenerationAction>,
    idx: usize,
    locked: bool,
}

impl<T: Clone> GenerationHistory<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            actions: Vec::new(),
            idx: 0,
            locked: false,
        }
    }

    fn plain_add(&mut self, item: T, action: GenerationAction) -> usize {
        self.items.push(item);
        self.actions.push(action);
        self.idx = self.items.len() - 1;
        self.idx
    }

    pub fn add(&mut self, item: T, action: GenerationAction) -> usize {
        if self.items.is_empty() || action != GenerationAction::Rewrite {
            return self.plain_add(item, action);
        }
        if self.actions.last() != Some(&GenerationAction::Rewrite) {
            return self.plain_add(item, action);
        }
        self.items[self.idx] = item;
        self.idx
    }

    pub fn previous(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        self.idx = self.idx.saturating_sub(1);
        Some(&self.items[self.idx])
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        self.idx = self.idx.min(self.items.len() - 1);
        Some(&self.items[self.idx])
    }
}