use std::fs;
use std::path::Path;

pub struct FilePreprocessor {
    path_to_file: String,
}

impl FilePreprocessor {
    pub fn new(path_to_file: String) -> Self {
        FilePreprocessor { path_to_file }
    }

    pub fn process_file(&self, text: &str) -> String {
        if self.is_python_file() {
            self.process_if_python(text)
        } else {
            text.to_string()
        }
    }

    fn is_python_file(&self) -> bool {
        self.path_to_file.ends_with(".py")
    }

    fn process_if_python(&self, text: &str) -> String {
        if self.contains_class_definition() {
            text.lines().map(|line| format!("    {}", line)).collect::<Vec<String>>().join("\n")
        } else {
            text.to_string()
        }
    }

    fn contains_class_definition(&self) -> bool {
        if let Ok(content) = fs::read_to_string(&self.path_to_file) {
            content.lines().any(|line| line.trim().starts_with("class "))
        } else {
            false
        }
    }
}