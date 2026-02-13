use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
pub  struct FilePreprocessor {
   pub path_to_file: String,
}

impl FilePreprocessor {
    pub fn new(path_to_file: String) -> Self {


        FilePreprocessor { path_to_file }
    }
   pub fn create(path_to_file: String) -> Self {
        FilePreprocessor { path_to_file }
    }   

    pub fn process_file(&self, text: &str) -> String {
        if self.is_python_file() && self.contains_class_definition() {
            return self.indent_text(text);
        }
        text.to_string()
    }

    fn is_python_file(&self) -> bool {
        self.path_to_file.ends_with(".py")
    }

    fn indent_text(&self, text: &str) -> String {
        text.lines().map(|line| format!("{}", line)).collect::<Vec<String>>().join("\n")
    }

    fn contains_class_definition(&self) -> bool {
        if let Ok(content) = self.read_file() {
            return content.contains("class ");
        }
        false
    }

    fn read_file(&self) -> io::Result<String> {
        let path = Path::new(&self.path_to_file);
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
// fn main() {
//     let preprocessor = FilePreprocessor::new("example.py".to_string());
//     let text = "class Example:\n    pass";
//     let processed_text = preprocessor.process_file(text);
//     println!("{}", processed_text);
// }