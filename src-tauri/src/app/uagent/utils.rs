// Import necessary crates
use log::{info, error};
use regex::Regex;
use serde_yaml::{self, Value};
use std::collections::HashMap;

// Function to load and parse YAML data
fn load_yaml(response_text: &str, keys_fix_yaml: &[&str]) -> HashMap<String, Value> {
    let response_text = response_text.trim().trim_start_matches("```yaml").trim_end_matches('`');
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

// Function to attempt fixing YAML formatting issues
fn try_fix_yaml(response_text: &str, keys_fix_yaml: &[&str]) -> HashMap<String, Value> {
    let response_text_lines: Vec<&str> = response_text.lines().collect();

    // First fallback - try to convert 'relevant line: ...' to 'relevant line: |-\n        ...'
    let mut response_text_lines_copy = response_text_lines.clone();
    for line in response_text_lines_copy.iter_mut() {
        for key in keys_fix_yaml {
            if line.contains(key) && !line.contains("|-") {
                *line = &line.replace(key, &format!("{} |-\n        ", key));
            }
        }
    }
    if let Ok(data) = serde_yaml::from_str(&response_text_lines_copy.join("\n")) {
        info!("Successfully parsed AI prediction after adding |-\n");
        return data;
    }

    // Second fallback - try to extract only range from first ```yaml to ````
    let snippet_pattern = Regex::new(r"```(yaml)?[\s\S]*?```").unwrap();
    if let Some(snippet) = snippet_pattern.find(&response_text_lines_copy.join("\n")) {
        let snippet_text = snippet.as_str().trim_start_matches("```yaml").trim_end_matches('`');
        if let Ok(data) = serde_yaml::from_str(snippet_text) {
            info!("Successfully parsed AI prediction after extracting yaml snippet");
            return data;
        }
    }

    // Third fallback - try to remove leading and trailing curly brackets
    let response_text_copy = response_text.trim().trim_start_matches('{').trim_end_matches('}').trim_end_matches(":\n");
    if let Ok(data) = serde_yaml::from_str(response_text_copy) {
        info!("Successfully parsed AI prediction after removing curly brackets");
        return data;
    }

    // Fourth fallback - try to remove last lines
    for i in 1..response_text_lines.len() {
        let response_text_lines_tmp = response_text_lines[..response_text_lines.len() - i].join("\n");
        if let Ok(data) = serde_yaml::from_str(&response_text_lines_tmp) {
            if data.get("language").is_some() {
                info!("Successfully parsed AI prediction after removing {} lines", i);
                return data;
            }
        }
    }

    // Fifth fallback - brute force
    if let Some(index_start) = response_text.find("\nlanguage:").or_else(|| response_text.find("language:")) {
        if let Some(index_last_code) = response_text.rfind("test_code:") {
            let index_end = response_text[index_last_code..].find("\n\n").unwrap_or(response_text.len());
            let response_text_copy = &response_text[index_start..index_end].trim();
            if let Ok(data) = serde_yaml::from_str(response_text_copy) {
                info!("Successfully parsed AI prediction when using the language: key as a starting point");
                return data;
            }
        }
    }

    HashMap::new()
}