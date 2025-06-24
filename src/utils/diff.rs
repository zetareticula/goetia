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
                &[a.place.clone(), a.plot_element.clone(), a.beat.clone()],
                &[b.place.clone(), b.plot_element.clone(), b.beat.clone()],
            )
        })
        .filter(|diff| !diff.is_empty())
        .collect();
    diffs.join("\n")
}

pub fn diff_prompt_change_dict(before: &HashMap<String, String>, after: &HashMap<String, String>) -> String {
    let keys_before: Vec<_> = before.keys().collect();
    let keys_after: Vec<_> = after.keys().collect();
    let diff_keys = diff_prompt_change_list(&keys_before, &keys_after);
    let values_before: Vec<_> = before.values().collect();
    let values_after: Vec<_> = after.values().collect();
    let diff_values = diff_prompt_change_list(&values_before, &values_after);
    format!("{}\n{}", diff_keys, diff_values).trim().to_string()
}