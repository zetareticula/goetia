// This file is part of the `utils` module, providing functions to extract elements from text.  
// It includes functions to extract elements between specified markers and to strip a specific end marker from text.
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, Visitor};
use std::fmt;
use std::str::FromStr;


#[derive(Debug, Serialize, Deserialize)]
pub fn extract_elements(text: &str, begin: &str, end: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut start = 0;
    while let Some(begin_pos) = text[start..].find(begin) {
        let begin_pos = start + begin_pos;
        if let Some(end_pos) = text[begin_pos..].find(end) {
            let end_pos = begin_pos + end_pos;
            let element = text[begin_pos + begin.len()..end_pos].trim().to_string();
            results.push(element);
            start = end_pos + end.len();
        } else {
            break;
        }
    }
    results
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub fn strip_remove_end(text: &str) -> String {
    let text = text.trim();
    let end_marker_stripped = "**END**".trim();
    if text.ends_with(end_marker_stripped) {
        text[..text.len() - end_marker_stripped.len()].trim().to_string()
    } else {
        text.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractedElements {
    pub elements: Vec<String>,
}

impl ExtractedElements {
    pub fn from_string(text: &str, begin: &str, end: &str) -> Self {
        let elements = extract_elements(text, begin, end);
        Self { elements }
    }
}

impl fmt::Display for ExtractedElements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.elements {
            write!(f, "{} ", element)?;
        }
        Ok(())
    }
}

impl FromStr for ExtractedElements {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_string(s, "**Element:** ", "**END**"))
    }
}