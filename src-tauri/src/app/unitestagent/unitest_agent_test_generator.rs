use std::fs;
use std::path::Path;
use std::process::Command;
use serde_yaml;
use std::path::PathBuf;


pub struct UnitestAgentTestGenerator {
    source_file_path: String,
    test_file_path: String,
    code_coverage_report_path: String,
    test_command: String,
    test_command_dir: String,
    llm_model: String,
    api_base: String,
    included_files: Vec<String>,
    coverage_type: String,
    desired_coverage: f64,
    additional_instructions: String,
    language: String,
    use_report_coverage_feature_flag: bool,
    // last_coverage_percentages: std::collections::HashMap<String, f64>,
    // current_coverage: f64,
    // Add other fields as needed
}



impl UnitestAgentTestGenerator {
    pub fn new(
        source_file_path: String,
        test_file_path: String,
        code_coverage_report_path: String,
        test_command: String,
        test_command_dir: String,
        llm_model: String,
        api_base: String,
        included_files: Vec<String>,
        coverage_type: String,
        desired_coverage: f64,
        additional_instructions: String,
        use_report_coverage_feature_flag: bool,
    ) -> Self {
       

        UnitestAgentTestGenerator {
            source_file_path,
            test_file_path,
            code_coverage_report_path,
            test_command,
            test_command_dir,
            llm_model,
            api_base,
            included_files,
            coverage_type,
            desired_coverage,
            additional_instructions,
            language: Self::get_code_language(&source_file_path),
            use_report_coverage_feature_flag,
            // last_coverage_percentages: std::collections::HashMap::new(),
            // current_coverage: 0.0,
            // Initialize other fields
        }
 
    }

    fn get_code_language(source_file_path: &String) -> String {
        // Implement language detection logic
        // This is a placeholder implementation
        // source_file_path.extension()
        //     .and_then(|ext| ext.to_str())
        //     .map(|ext| ext.to_lowercase())
        //     .unwrap_or_else(|| "unknown".to_string())
        return "c".to_string();
    }


    pub fn initial_test_suite_analysis(&mut self) -> Result<()> {
        const ALLOWED_ATTEMPTS: u8 = 3;

        // Analyze test headers indentation
        let test_headers_indentation = self.analyze_test_headers_indentation(ALLOWED_ATTEMPTS)?;

        // Analyze relevant line numbers for inserting tests and imports
        let (relevant_line_number_to_insert_tests_after, relevant_line_number_to_insert_imports_after) =
            self.analyze_insert_line_numbers(ALLOWED_ATTEMPTS)?;

        // Set the analyzed values
        self.test_headers_indentation = test_headers_indentation;
        self.relevant_line_number_to_insert_tests_after = relevant_line_number_to_insert_tests_after;
        self.relevant_line_number_to_insert_imports_after = relevant_line_number_to_insert_imports_after;

        info!("Test headers indentation: {}", self.test_headers_indentation);

        Ok(())
    }
    pub fn run_coverage(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Running build/test command to generate coverage report: \"{}\"", self.test_command);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.test_command)
            .current_dir(&self.test_command_dir)
            .output()?;

        if !output.status.success() {
            return Err(format!("Fatal: Error running test command. Are you sure the command is correct? \"{}\"\nExit code {:?}. \nStdout: \n{} \nStderr: \n{}",
                self.test_command,
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)).into());
        }

        // Implement coverage processing logic here
        // This will depend on how you've implemented CoverageProcessor in Rust

        Ok(())
    }

}