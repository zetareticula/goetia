use std::fmt;
use std::str::FromStr;
use crate::utils::extract::extract_elements;


const TITLE_ELEMENT: &str = "Title: ";
const END_MARKER: &str = "**END**";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Title {
    pub title: String,
}

impl Title {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn from_string(text: &str) -> Self {
        let titles = extract_elements(text, TITLE_ELEMENT, END_MARKER);
        Self {
            title: titles.first().cloned().unwrap_or_default(),
        }
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", TITLE_ELEMENT, self.title, END_MARKER)
    }
}

impl FromStr for Title {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_string(s))
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Titles {
    pub titles: Vec<Title>,
}

impl Titles {
    pub fn from_string(text: &str) -> Self {
        let titles = extract_elements(text, TITLE_ELEMENT, END_MARKER);
        let titles = titles.into_iter().map(Title::new).collect();
        Self { titles }
    }
}

impl fmt::Display for Titles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for title in &self.titles {
            s.push_str(&format!("{}{}{}", TITLE_ELEMENT, title.title, END_MARKER));
        }
        write!(f, "{}", s)
    }
}

impl FromStr for Titles {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_string(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_from_string() {
        let text = "Title: My First Title **END**";
        let title = Title::from_string(text);
        assert_eq!(title.title, "My First Title");
    }

    #[test]
    fn test_titles_from_string() {
        let text = "Title: First Title **END**Title: Second Title **END**";
        let titles = Titles::from_string(text);
        assert_eq!(titles.titles.len(), 2);
        assert_eq!(titles.titles[0].title, "First Title");
        assert_eq!(titles.titles[1].title, "Second Title");
    }
}