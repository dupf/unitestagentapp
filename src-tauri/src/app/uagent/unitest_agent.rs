// use crate::report_generator::ReportGenerator;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;

use tauri::{AppHandle, Manager};

use crate::app::uagent::{
    custom_logger::CustomLogger, report_generator::ReportGenerator,
    unitest_agent_test_generator::TestDetails, unitest_agent_test_generator::TestYaml,
    unitest_agent_test_generator::UnitTestAgentTestGenerator,
};

pub struct UnitestAgent {
    source_file_path: String,
    test_file_path: String,
    test_file_output_path: String,
    code_coverage_report_path: String,
    test_command: String,
    test_command_dir: String,
    included_files: Option<Vec<String>>,
    coverage_type: String,
    report_filepath: String,
    desired_coverage: i32,
    max_iterations: i32,
    additional_instructions: String,
    model: String,
    isremote: bool,
    api_base: String,
    strict_coverage: bool,
    run_tests_multiple_times: i32,
    use_report_coverage_feature_flag: bool,
    test_gen: UnitTestAgentTestGenerator,
}

impl UnitestAgent {
    pub fn new(
        source_file_path: String,
        test_file_path: String,
        test_file_output_path: String,
        code_coverage_report_path: String,
        test_command: String,
        test_command_dir: String,
        included_files: Option<Vec<String>>,
        coverage_type: String,
        report_filepath: String,
        desired_coverage: i32,
        max_iterations: i32,
        additional_instructions: String,
        model: String,
        isremote: bool,
        api_base: String,
        strict_coverage: bool,
        run_tests_multiple_times: i32,
        use_report_coverage_feature_flag: bool, // report_filepath: String,
    ) -> Self {
        let test_gen = UnitTestAgentTestGenerator::new(
            &source_file_path,
            &test_file_path,
            &code_coverage_report_path,
            &test_command,
            Some(&test_command_dir),
            included_files.as_ref(),
            Some(&coverage_type),
            Some(desired_coverage),
            &model,
            &api_base,
            &additional_instructions,
            use_report_coverage_feature_flag,
            isremote,
        );

        let unitest_agent: UnitestAgent = UnitestAgent {
            source_file_path,
            test_file_path,
            test_file_output_path,
            code_coverage_report_path,
            test_command,
            test_command_dir,
            included_files,
            coverage_type,
            report_filepath,
            desired_coverage,
            max_iterations,
            additional_instructions,
            model,
            isremote,
            api_base,
            strict_coverage,
            run_tests_multiple_times,
            use_report_coverage_feature_flag,
            test_gen: test_gen.unwrap(),
        };
        unitest_agent.validate_paths();
        // unitest_agent.duplicate_test_file();
        // unitest_agent.duplicate_test_file();
        unitest_agent
    }

    fn validate_paths(&self) {
        if !Path::new(&self.source_file_path).is_file() {
            File::create(&self.source_file_path)
                .expect("Failed to create source file")
                .write_all(b"")
                .expect("Failed to write to source file");
            println!(
                "Source file not found. Created an empty file at {}",
                self.source_file_path
            );
        }

        if !Path::new(&self.test_file_path).is_file() {
            File::create(&self.test_file_path)
                .expect("Failed to create test file")
                .write_all(b"")
                .expect("Failed to write to test file");
            println!(
                "Test file not found. Created an empty file at {}",
                self.test_file_path
            );
        }
    }
    fn duplicate_test_file(&mut self) {
        if !self.test_file_output_path.is_empty() {
            fs::copy(&self.test_file_path, &self.test_file_output_path)
                .expect("Failed to copy test file");
        } else {
            self.test_file_output_path = self.test_file_path.clone();
        }
    }
    pub async fn run(&mut self, handle: AppHandle, id: u64) {
        let mut iteration_count = 0;
        // println!("==self.test_gen:== ======");
        // self.test_gen
        //     .initial_test_suite_analysis(handle.clone(), id)
        //     .await;
        let mut test_results_list: Vec<TestDetails> = Vec::new();

        let finish_reason: String = "finish".to_string();
        let role: String = "user".to_string();

        while iteration_count < self.max_iterations {
            let generated_tests_result =
                self.test_gen.generate_tests(handle.clone(), id, 4096, false);
        
            let generated_tests_result_vec: Vec<TestDetails> = match generated_tests_result.await {
                Ok(tests) => tests,
                Err(e) => {
                    println!("Error generating tests: {}", e);
                    continue;
                }
            };

            // println!("generated_tests_result_vec: {:?}", generated_tests_result_vec);

            for generated_test in generated_tests_result_vec.into_iter() {
                if test_results_list.is_empty() {
                
                    test_results_list.push(generated_test);
                } else {
                    let mut found = false;
                    for list_test in &test_results_list {
                        if list_test.test_number == generated_test.test_number {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        test_results_list.push(generated_test);
                    }
                }
            }
            iteration_count += 1;
        }

        // // 生成静态安全检查结果
        let generated_tests_result =
            self.test_gen
                .generate_static_sec(handle.clone(), id, 4096, false);

        let static_sec_result = match generated_tests_result.await {
            Ok(result) => result,
            Err(e) => {
                println!("Error generating static sec: {}", e);
                return;
            }
        };

        for issue in static_sec_result.coding_standard_issues.iter() {
            println!("Coding Standard Issue: {:?}", issue);
            println!("Issue ID: {:?}", issue.issue_id);
            println!("Category: {:?}", issue.category);
            println!("Severity: {:?}", issue.severity);
            println!("Description: {:?}", issue.description);
            println!("Location: {:?}", issue.location);
            println!("Impact: {:?}", issue.impact);
            println!("Recommendation: {:?}", issue.recommendation);
            println!("Best Practice: {:?}", issue.best_practice);
        }

        for issue in static_sec_result.performance_issues.iter() {
            println!("Performance Issue: {:?}", issue);
            println!("Issue ID: {:?}", issue.issue_id);
            println!("Category: {:?}", issue.category);
            println!("Severity: {:?}", issue.severity);
            println!("Description: {:?}", issue.description);
            println!("Location: {:?}", issue.location);
            println!("Impact: {:?}", issue.impact);
            println!("Recommendation: {:?}", issue.recommendation);
            println!("Best Practice: {:?}", issue.best_practice);
        }

        for issue in static_sec_result.security_vulnerabilities.iter() {
            println!("Security Vulnerability: {:?}", issue);
            println!("Issue ID: {:?}", issue.issue_id);
            println!("Category: {:?}", issue.category);
            println!("Severity: {:?}", issue.severity);
            println!("Description: {:?}", issue.description);
            println!("Location: {:?}", issue.location);
            println!("Impact: {:?}", issue.impact);
            println!("Recommendation: {:?}", issue.recommendation);
            println!("Best Practice: {:?}", issue.best_practice);
        }

        // 打印静态检查的详细信息
        // ReportGenerator::generate_report(handle.clone(), &test_results_list, &self.report_filepath)
        //     .await;

        // ReportGenerator::generate_static_analysis_report(handle.clone(), &static_sec_result).await;
        ReportGenerator::generate_combined_report(handle.clone(), &test_results_list, &static_sec_result).await; 

        
    }
}
