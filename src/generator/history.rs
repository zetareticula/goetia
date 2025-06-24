#[derive(Debug, Clone, PartialEq)]
pub enum GenerationAction {
    New,
    Continue,
    Rewrite,
}

#[derive(Debug, Clone)]
pub struct GenerationHistory<T: Clone> {
    items: Vec<T>,
    actions: Vec<GenerationAction>,
    idx: i32,
}

impl<T: Clone> GenerationHistory<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            actions: Vec::new(),
            idx: -1,
        }
    }

    pub fn add(&mut self, item: T, action: GenerationAction) -> i32 {
        if self.items.is_empty() || action != GenerationAction::Rewrite {
            return self.plain_add(item, action);
        }
        if let Some(last_action) = self.actions.last() {
            if *last_action != GenerationAction::Rewrite {
                return self.plain_add(item, action);
            }
        }
        self.items[self.idx as usize] = item;
        self.idx
    }

    fn plain_add(&mut self, item: T, action: GenerationAction) -> i32 {
        self.items.push(item);
        self.actions.push(action);
        self.idx = (self.items.len() - 1) as i32;
        self.idx
    }

    pub fn previous(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        self.idx = self.idx.max(0);
        Some(&self.items[self.idx as usize])
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        self.idx = self.idx.min(self.items.len() as i32 - 1);
        Some(&self.items[self.idx as usize])
    }
}