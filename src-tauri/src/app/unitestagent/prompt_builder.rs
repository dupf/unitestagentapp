use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use log::error;
use std::collections::HashMap;
use tera::{Tera, Context};
use crate::app::unitestagent::settings::config_loader::get_settings;
use config::{Config};

const MAX_TESTS_PER_RUN: u32 = 4;

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

pub struct PromptBuilder {
    source_file_name: String,
    test_file_name: String,
    source_file: String,
    test_file: String,
    code_coverage_report: String,
    language: String,
    source_file_numbered: String,
    test_file_numbered: String,
    included_files: String,
    additional_instructions: String,
    failed_test_runs: String,
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
    ) -> io::Result<Self> {
        let source_file_name = Path::new(source_file_path).file_name().unwrap().to_str().unwrap().to_string();
        let test_file_name = Path::new(test_file_path).file_name().unwrap().to_str().unwrap().to_string();
        let source_file = Self::read_file(source_file_path)?;
        let test_file = Self::read_file(test_file_path)?;

        let source_file_numbered = source_file.lines().enumerate()
            .map(|(i, line)| format!("{} {}", i + 1, line))
            .collect::<Vec<_>>()
            .join("\n");

        let test_file_numbered = test_file.lines().enumerate()
            .map(|(i, line)| format!("{} {}", i + 1, line))
            .collect::<Vec<_>>()
            .join("\n");

        let included_files = if !included_files.is_empty() {
            ADDITIONAL_INCLUDES_TEXT.replace("{included_files}", included_files)
        } else {
            String::new()
        };

        let additional_instructions = if !additional_instructions.is_empty() {
            ADDITIONAL_INSTRUCTIONS_TEXT.replace("{additional_instructions}", additional_instructions)
        } else {
            String::new()
        };

        let failed_test_runs = if !failed_test_runs.is_empty() {
            FAILED_TESTS_TEXT.replace("{failed_test_runs}", failed_test_runs)
        } else {
            String::new()
        };

        Ok(Self {
            source_file_name,
            test_file_name,
            source_file,
            test_file,
            code_coverage_report: code_coverage_report.to_string(),
            language: language.to_string(),
            source_file_numbered,
            test_file_numbered,
            included_files,
            additional_instructions,
            failed_test_runs,
        })
    }

    fn read_file(file_path: &str) -> io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn build_prompt(&self) -> Result<(String, String), tera::Error> {
        let mut context = Context::new();
        context.insert("source_file_name", &self.source_file_name);
        context.insert("test_file_name", &self.test_file_name);
        context.insert("source_file_numbered", &self.source_file_numbered);
        context.insert("test_file_numbered", &self.test_file_numbered);
        context.insert("source_file", &self.source_file);
        context.insert("test_file", &self.test_file);
        context.insert("code_coverage_report", &self.code_coverage_report);
        context.insert("additional_includes_section", &self.included_files);
        context.insert("failed_tests_section", &self.failed_test_runs);
        context.insert("additional_instructions_text", &self.additional_instructions);
        context.insert("language", &self.language);
        context.insert("max_tests", &MAX_TESTS_PER_RUN);
        let mut tera = Tera::default();
        // Get the system template from the configuration
        let settings = get_settings();
        let test_generation_prompt_htzr_cn = settings.get_test_generation_prompt_htzr_cn();
        // let system_template = test_generation_prompt_htzr_cn
        //     .and_then(|config| config.get("system"))
        //     .and_then(|v| v.as_str())
        //     .unwrap_or_default()
        //     .to_string();
        // let user_template = config.get("user")
        //     .and_then(|v| v.as_str())
        //     .unwrap_or_default()    
        //     .to_string();
        tera.add_raw_template("system", &system_template)?;
        tera.add_raw_template("user", &user_template)?;
        let system_prompt = tera.render("system", &context)?;
        let user_prompt = tera.render("user", &context)?;
 
        Ok((system_prompt, user_prompt))
    }

    pub fn build_prompt_custom(&self, file: &str) -> Result<(String, String), tera::Error> {
        let mut context = Context::new();
        // ... (same as build_prompt)
        let mut tera = Tera::default();
        // Assume these templates are loaded from somewhere
        // let system_template =Settings::get_settings().get(file).system;
        // let user_template = Settings::get_settings().get(file).user;
        // let settings = Settings::get_settings();
        let settings = get_settings();
        let system_template = settings.test_generation_prompt_htzr_cn.system;
        let user_template = settings.test_generation_prompt_htzr_cn.user;
        tera.add_raw_template("system", &system_template)?;
        tera.add_raw_template("user", &user_template)?;

        let system_prompt = tera.render("system", &context)?;
        let user_prompt = tera.render("user", &context)?;

        Ok((system_prompt, user_prompt))
    }
}
