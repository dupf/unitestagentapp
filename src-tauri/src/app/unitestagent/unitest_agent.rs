use std::fs;
use std::path::Path;
use log::{info, error};
use crate::app::unitestagent::custom_logger::CustomLogger;
use crate::app::unitestagent::reportgenerator::ReportGenerator;
use crate::app::unitestagent::unitest_agent_test_generator::UnitestAgentTestGenerator;
use std::env;

pub struct UnitestAgent {
    // args: env::Args,
    source_file_path: String,
    test_file_path: String,
    code_coverage_report_path: String,
    test_command: String,
    test_command_dir: String,
    included_files: Vec<String>,
    coverage_type: String,
    desired_coverage: f64,
    model: String,
    api_base: String,
    logger: CustomLogger,
    test_gen: UnitestAgentTestGenerator,
    report_generator: ReportGenerator,
}

impl UnitestAgent {
    pub fn new(
        source_file_path: String,
        test_file_path: String,
        code_coverage_report_path: String,
        test_command: String,
        test_command_dir: String,
        included_files: Vec<String>,
        coverage_type: String,
        desired_coverage: f64,
        llm_model: String,
        api_base: String,
        additional_instructions: String,
    )  {
        
       
        
        let test_gen = UnitestAgentTestGenerator::new(source_file_path, test_file_path, code_coverage_report_path, test_command, 
            test_command_dir, llm_model, api_base, included_files, coverage_type,
             desired_coverage, additional_instructions, true);

        // let report_generator = ReportGenerator::new(code_coverage_report_path, test_command, test_command_dir);
        

        // UnitestAgentTestGenerator::new already has all necessary arguments
        // No need to call it again or add more arguments
        //     code_coverage_report_path,
        //     test_command,   
        //     test_command_dir,   
        //     included_files,
        //     coverage_type,
        //     desired_coverage,   
        //     model,  
        //     api_base
        // );
        // Ok(Self { args, logger, test_gen })

    }

    fn validate_paths(&mut self) -> Result<(), std::io::Error> {

        if !Path::new(&self.source_file_path).is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Source file not found at {}", self.source_file_path),
            ));
        }
        if !Path::new(&self.test_file_path).is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Test file not found at {}", self.test_file_path),
            ));
        }
        Ok(())
    }

    // fn duplicate_test_file(&mut self) -> Result<(), std::io::Error> {

    //     if !self.test_file_output_path.is_empty() {
    //         self.test_file_path = self.test_file_output_path.clone();
    //     } else {
    //         // self.test_file_output_path = self.test_file_path.clone();
    //         self.test_file_output_path = self.test_file_output_path.clone();
    //     }
    //     Ok(())
    // }
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut iteration_count = 0;
        let mut test_results_list = Vec::new();
        // self.test_gen.initial_test_suite_analysis()?;
        self.test_gen.initial_test_suite_analysis()?;

        while iteration_count < 2 {
            // self.logger.info!(
            //     "Current Coverage: {:.2}%",
            //     self.test_gen.current_coverage * 100.0
            // );
            // info!("Desired Coverage: {}%", self.test_gen.desired_coverage);
            info!("Desired Coverage: ");
            let generated_tests_dict = self.test_gen(4096)?;

            println!("===== generated_tests_dict output =====\n{:?}", generated_tests_dict);

            for generated_test in generated_tests_dict.get("new_tests").unwrap_or(&Vec::new()) {
                println!("===== generated_test output =====\n{:?}", generated_test);
                if test_results_list.is_empty() {
                    test_results_list.push(generated_test.clone());
                } else {
                    if !test_results_list.iter().any(|list_test| list_test["测试用例编号"] == generated_test["测试用例编号"]) {
                        test_results_list.push(generated_test.clone());
                    }
                }
            }
            iteration_count += 1;
        }

        if self.test_gen.current_coverage >= (self.test_gen.desired_coverage / 100.0) {
            info!(
                "Reached above target coverage of {}% (Current Coverage: {:.2}%) in {} iterations.",
                self.test_gen.desired_coverage,
                self.test_gen.current_coverage * 100.0,
                iteration_count
            );
        } else if iteration_count == self.max_iterations {
            let failure_message = format!(
                "Reached maximum iteration limit without achieving desired coverage. Current Coverage: {:.2}%",
                self.test_gen.current_coverage * 100.0
            );
            if self.args.strict_coverage {
                error!("{}", failure_message);
                std::process::exit(2);
            } else {
                info!("{}", failure_message);
            }
        }

        info!(
            "Total number of input tokens used for LLM model {}: {}",
            self.test_gen.llm_caller.model, self.test_gen.total_input_token_count
        );
        info!(
            "Total number of output tokens used for LLM model {}: {}",
            self.test_gen.llm_caller.model, self.test_gen.total_output_token_count
        );
        
        self.report_generator.generate_report(&test_results_list, &self.args.report_path)?;



        Ok(())
    }


}