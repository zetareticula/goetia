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

pub fn strip_remove_end(text: &str) -> String {
    let text = text.trim();
    let end_marker_stripped = "**END**".trim();
    if text.ends_with(end_marker_stripped) {
        text[..text.len() - end_marker_stripped.len()].trim().to_string()
    } else {
        text.to_string()
    }
}