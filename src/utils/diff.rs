use std::collections::HashMap;

use crate::models::Scene;

pub fn diff_prompt_change_str(before: &str, after: &str) -> String {
    let mut diff = String::new();
    let before_lines: Vec<&str> = before.split('\n').collect();
    let after_lines: Vec<&str> = after.split('\n').collect();
    let differ = diff::lines(before, after);
    for line in differ {
        match line {
            diff::Result::Left(l) => diff.push_str(&format!("-{}\n", l)),
            diff::Result::Right(r) => diff.push_str(&format!("+{}\n", r)),
            diff::Result::Both(_, _) => {}
        }
    }
    diff.trim_end().to_string()
}

pub fn diff_prompt_change_list(before: &[String], after: &[String]) -> String {
    if before.len() > after.len() {
        return "Deleted element".to_string();
    }
    if before.len() < after.len() {
        return "Added new element".to_string();
    }
    let diffs: Vec<String> = before
        .iter()
        .zip(after.iter())
        .map(|(a, b)| diff_prompt_change_str(a, b))
        .filter(|diff| !diff.is_empty())
        .collect();
    diffs.join("\n")
}

pub fn diff_prompt_change_scenes(before: &[Scene], after: &[Scene]) -> String {
    if before.len() > after.len() {
        return "Deleted element".to_string();
    }
    if before.len() < after.len() {
        return "Added new element".to_string();
    }
    let diffs: Vec<String> = before
        .iter()
        .zip(after.iter())
        .map(|(a, b)| {
            diff_prompt_change_list(
                &[a.name.clone(), a.description.clone()],
                &[b.name.clone(), b.description.clone()],
            )
        })
        .filter(|diff| !diff.is_empty())
        .collect();
    diffs.join("\n")
}

pub fn diff_prompt_change_dict(before: &HashMap<String, String>, after: &HashMap<String, String>) -> String {
    let keys_before: Vec<String> = before.keys().cloned().collect();
    let keys_after: Vec<String> = after.keys().cloned().collect();
    let diff_keys = diff_prompt_change_list(&keys_before, &keys_after);
    let values_before: Vec<String> = before.values().cloned().collect();
    let values_after: Vec<String> = after.values().cloned().collect();
    let diff_values = diff_prompt_change_list(&values_before, &values_after);
    format!("{}\n{}", diff_keys, diff_values).trim().to_string()
}