use std::fmt::Debug;
use std::any::Any;






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

pub trait FilterApi: Send + Sync + 'static {
    fn filter(&self, text: &str) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_enum(&self) -> FilterApiEnum;
}

#[derive(Clone)]
pub struct MockFilterApi;

impl Debug for MockFilterApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockFilterApi")
    }
}

impl FilterApi for MockFilterApi {
    fn filter(&self, _text: &str) -> bool {
        true
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_enum(&self) -> FilterApiEnum {
        FilterApiEnum::Mock(self.clone())
    }
}

#[derive(Clone)]
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
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_enum(&self) -> FilterApiEnum {
        FilterApiEnum::Perspective(self.clone())
    }
}

impl From<&dyn FilterApi> for FilterApiEnum {
    fn from(api: &dyn FilterApi) -> Self {
        api.as_enum()
    }
}

impl From<FilterApiEnum> for Box<dyn FilterApi> {
    fn from(api: FilterApiEnum) -> Self {
        match api {
            FilterApiEnum::Mock(inner) => Box::new(inner),
            FilterApiEnum::Perspective(inner) => Box::new(inner),
        }
    }
}

impl Clone for Box<dyn FilterApi> {
    fn clone(&self) -> Self {
        self.as_enum().into()
    }
}

impl FilterApi for FilterApiEnum {
    fn filter(&self, text: &str) -> bool {
        match self {
            FilterApiEnum::Mock(api) => api.filter(text),
            FilterApiEnum::Perspective(api) => api.filter(text),
        }
    }
    
    fn as_any(&self) -> &dyn Any {
        match self {
            FilterApiEnum::Mock(api) => <dyn FilterApi>::as_any(api),
            FilterApiEnum::Perspective(api) => <dyn FilterApi>::as_any(api),
        }
    }
    
    fn as_enum(&self) -> FilterApiEnum {
        self.clone()
    }
}

impl Clone for FilterApiEnum {
    fn clone(&self) -> Self {
        match self {
            Self::Mock(inner) => Self::Mock(inner.clone()),
            Self::Perspective(inner) => Self::Perspective(inner.clone()),
        }
    }
}

pub trait AsAny: 'static {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}