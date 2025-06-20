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