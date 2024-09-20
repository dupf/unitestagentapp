use log::info;
use regex::Regex;
use serde_yaml::Value;
use std::collections::HashMap;

pub fn load_yaml(response_text: &str, keys_fix_yaml: &[String]) -> HashMap<String, serde_yaml::Value> {
    let response_text = response_text.trim().strip_prefix("```yaml").unwrap_or(response_text).trim_end_matches('`');
    match serde_yaml::from_str(response_text) {
        Ok(data) => data,
        Err(e) => {
            info!("Failed to parse AI prediction: {}. Attempting to fix YAML formatting.", e);
            let data = try_fix_yaml(response_text, keys_fix_yaml);
            if data.is_empty() {
                info!("Failed to parse AI prediction after fixing YAML formatting.");
            }
            data
        }
    }
}

pub fn try_fix_yaml(response_text: &str, keys_fix_yaml: &[String]) -> HashMap<String, Value> {
    let response_text_lines: Vec<&str> = response_text.lines().collect();

    // First fallback
    let mut response_text_lines_copy = response_text_lines.clone();
    for (i, line) in response_text_lines_copy.iter_mut().enumerate() {
        for key in keys_fix_yaml {
            if line.contains(key) && !line.contains("|-") {
                let new_line = format!("{} |-\n        ", key);
                *line = line.replace(key, &new_line);
            }
        }
    }
    if let Ok(data) = serde_yaml::from_str(&response_text_lines_copy.join("\n")) {
        info!("Successfully parsed AI prediction after adding |-");
        return data;
    }

    // Second fallback
    let snippet_pattern = Regex::new(r"```(yaml)?[\s\S]*?```").unwrap();
    if let Some(snippet) = snippet_pattern.find(&response_text_lines_copy.join("\n")) {
        let snippet_text = snippet.as_str().trim_start_matches("```yaml").trim_end_matches("`");
        if let Ok(data) = serde_yaml::from_str(snippet_text) {
            info!("Successfully parsed AI prediction after extracting yaml snippet");
            return data;
        }
    }

    // Third fallback
    let response_text_copy = response_text.trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim_end_matches(":\n");
    if let Ok(data) = serde_yaml::from_str(response_text_copy) {
        info!("Successfully parsed AI prediction after removing curly brackets");
        return data;
    }

    // Fourth fallback
    for i in 1..response_text_lines.len() {
        let response_text_lines_tmp = response_text_lines[..response_text_lines.len() - i].join("\n");
        if let Ok(data) = serde_yaml::from_str(&response_text_lines_tmp) {
            let data_map: HashMap<String, Value> = data;
            if data_map.contains_key("language") {
                info!("Successfully parsed AI prediction after removing {} lines", i);
                return data_map;
            }
        }
    }

    // Fifth fallback
    let index_start = response_text.find("\nlanguage:")
        .or_else(|| response_text.find("language:"))
        .unwrap_or(0);
    let index_last_code = response_text.rfind("test_code:").unwrap_or(response_text.len());
    let index_end = response_text[index_last_code..].find("\n\n")
        .map(|i| i + index_last_code)
        .unwrap_or(response_text.len());
    let response_text_copy = &response_text[index_start..index_end].trim();
    if let Ok(data) = serde_yaml::from_str(response_text_copy) {
        info!("Successfully parsed AI prediction when using the language: key as a starting point");
        return data;
    }

    HashMap::new()
}