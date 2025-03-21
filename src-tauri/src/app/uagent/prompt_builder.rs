// main.rs
use log::error;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use tera::{Context, Tera};

use crate::app::uagent::config_setting::PromptConfig;

const MAX_TESTS_PER_RUN: usize = 4;

const ADDITIONAL_INCLUDES_TEXT: &str = r#"
## Additional Includes
The following is a set of included files used as context for the source code above. This is usually included libraries needed as context to write better tests:
======
{included_files}
======
"#;

const ADDITIONAL_INSTRUCTIONS_TEXT: &str = r#"
## Additional Instructions
======
{additional_instructions}
======
"#;

const FAILED_TESTS_TEXT: &str = r#"
## Previous Iterations Failed Tests
Below is a list of failed tests that you generated in previous iterations. Do not generate the same tests again, and take the failed tests into account when generating new tests.
======
{failed_test_runs}
======
"#;
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    source_file_name: String,
    test_file_name: String,
    source_file: String,
    test_file: String,
    source_file_numbered: String,
    test_file_numbered: String,
    code_coverage_report: String,
    included_files: String,
    additional_instructions: String,
    failed_test_runs: String,
    language: String,
}

impl PromptBuilder {
    pub fn new(
        source_file_path: &str,
        test_file_path: &str,
        code_coverage_report: &str,
        included_files: &str,
        additional_instructions: &str,
        failed_test_runs: &str,
        language: &str,
    ) -> PromptBuilder {
        let source_file_name = Path::new(source_file_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        // println!("source_file_name: {}", source_file_path);
        // println!("source_file_path ==={}", source_file_path);
        let test_file_name = Path::new(test_file_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        
        // println!("\n=== Context Variables ==={}", source_file_path);
        let source_file = Self::read_file(source_file_path).unwrap();
        let test_file = Self::read_file(test_file_path).unwrap();

        let source_file_numbered: String = source_file
            .lines()
            .enumerate()
            .map(|(i, line)| format!("{} {}", i + 1, line))
            .collect::<Vec<String>>()
            .join("\n");

        let test_file_numbered = test_file
            .lines()
            .enumerate()
            .map(|(i, line)| format!("{} {}", i + 1, line))
            .collect::<Vec<String>>()
            .join("\n");

        let included_files = if !included_files.is_empty() {
            ADDITIONAL_INCLUDES_TEXT.replace("{included_files}", included_files)
        } else {
            String::new()
        };

        let additional_instructions = if !additional_instructions.is_empty() {
            ADDITIONAL_INSTRUCTIONS_TEXT
                .replace("{additional_instructions}", additional_instructions)
        } else {
            String::new()
        };

        let failed_test_runs = if !failed_test_runs.is_empty() {
            FAILED_TESTS_TEXT.replace("{failed_test_runs}", failed_test_runs)
        } else {
            String::new()
        };


        PromptBuilder {
            source_file_name,
            test_file_name,
            source_file,
            test_file,
            source_file_numbered,
            test_file_numbered,
            code_coverage_report: code_coverage_report.to_string(),
            included_files,
            additional_instructions,
            failed_test_runs,
            language: language.to_string(),
        }
    }

    fn read_file(file_path: &str) -> io::Result<String> {
        if !Path::new(file_path).exists() {
            File::create(file_path)?;
        }
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(e) => {
                error!("Failed to read file {}: {}", file_path, e);
                Err(e)
            }
        }
    }

    pub fn build_prompt(&self) -> Result<(String, String), tera::Error> {
        let mut context = Context::new();
        context.insert("source_file_name", &self.source_file_name);
        context.insert("test_file_name", &self.test_file_name);
        context.insert("source_file", &self.source_file);
        context.insert("test_file", &self.test_file);
        context.insert("source_file_numbered", &self.source_file_numbered);
        context.insert("test_file_numbered", &self.test_file_numbered);
        context.insert("code_coverage_report", &self.code_coverage_report);
        context.insert("additional_includes_section", &self.included_files);
        context.insert("failed_tests_section", &self.failed_test_runs);
        context.insert(
            "additional_instructions_text",
            &self.additional_instructions,
        );
        context.insert("language", &self.language);
        context.insert("max_tests", &MAX_TESTS_PER_RUN);

        let config: PromptConfig =
            PromptConfig::from_file("src/test_generation_prompt.toml").unwrap();
        let mut tera: Tera = Tera::default();

        tera.add_raw_template(
            "system_template",
            &config.get_system_prompt("test_generation_prompt").unwrap(),
        )?;
        tera.add_raw_template(
            "user_template",
            &config.get_user_prompt("test_generation_prompt").unwrap(),
        )?;

        // println!("\n=== Context Variables ===");
        // for (key, value) in context.clone().into_json().as_object().unwrap() {
        //     println!("{}: {:?}", key, value);
        // }

        let system_prompt: String = tera.render("system_template", &context)?;
        let user_prompt: String = tera.render("user_template", &context)?;

        // println!("system_prompt: {}", system_prompt);
        // println!("user_prompt####################: {}", user_prompt);
        Ok((system_prompt, user_prompt))
    }

    pub fn build_prompt_custom(
        &self,
        filename: &str,
        section: &str,
    ) -> Result<(String, String), tera::Error> {

        let mut context = Context::new();
        context.insert("source_file_name", &self.source_file_name);
        context.insert("test_file_name", &self.test_file_name);
        context.insert("source_file", &self.source_file);
        context.insert("test_file", &self.test_file);
        context.insert("source_file_numbered", &self.source_file_numbered);
        context.insert("test_file_numbered", &self.test_file_numbered);
        context.insert("code_coverage_report", &self.code_coverage_report);
        context.insert("additional_includes_section", &self.included_files);
        context.insert("failed_tests_section", &self.failed_test_runs);
        context.insert(
            "additional_instructions_text",
            &self.additional_instructions,
        );
        context.insert("language", &self.language);
        context.insert("max_tests", &MAX_TESTS_PER_RUN);
        
        // println!("==context:== {:?}", context);
        println!("==filename:== {:?}", filename);
        println!("==section:== {:?}", section);
        let config: PromptConfig = PromptConfig::from_file(filename).unwrap();
        let mut tera: Tera = Tera::default();

        tera.add_raw_template(
            "system_template",
            &config.get_system_prompt(section).unwrap(),
        )?;
        tera.add_raw_template("user_template", &config.get_user_prompt(section).unwrap())?;

        for (key, value) in context.clone().into_json().as_object().unwrap() {
            println!("{}: {:?}", key, value);
        }
        let system_prompt: String = tera.render("system_template", &context)?;
        let user_prompt: String = tera.render("user_template", &context)?;

        Ok((system_prompt, user_prompt))
    }
}

// fn main() {
//     // Example usage
// }
