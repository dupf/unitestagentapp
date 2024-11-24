
mod ai_caller;
use crate::ai_caller::AICaller;
pub mod file_preprocessor;
use file_preprocessor::FilePreprocessor;
mod prompt_builder;
mod runner;
use langchain_rust::agent;
use runner::Runner;
use clap::Parser;
use std::env;
pub mod report_generator;
pub mod config_setting;
use crate::prompt_builder::PromptBuilder;
pub mod unitest_agent_test_generator;
mod unitest_agent;
use crate::unitest_agent::UnitestAgent;
mod custom_logger;


#[derive(Parser, Debug)]
#[command(name = "Unittest Agent")]
pub struct Args {
    #[arg(long, default_value = "/Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/twofen.c")]
    source_file_path: String,

    #[arg(long, default_value = "/Users/mac/Documents/work/htzr/bbaa.c")]
    test_file_path: String,

    #[arg(long, default_value = "")]
    test_file_output_path: String,

    #[arg(long, default_value = "")]
    code_coverage_report_path: String,

    #[arg(long, default_value = "./")]
    test_command: String,

    #[arg(long, default_value_t = env::current_dir().unwrap().to_str().unwrap().to_string())]
    test_command_dir: String,

    #[arg(long)]
    included_files: Option<Vec<String>>,

    #[arg(long, default_value = "lcov")]
    coverage_type: String,

    #[arg(long, default_value = "/Users/mac/Documents/work/htzr/unitest_agent/unitestagenttools/test_results.html")]
    report_filepath: String,

    #[arg(long, default_value_t = 90)]
    desired_coverage: i32,

    #[arg(long, default_value_t = 1)]
    max_iterations: i32,

    #[arg(long, default_value = "")]
    additional_instructions: String,

    #[arg(long, default_value = "deepseek-coder")]
    model: String,

    #[arg(long, default_value_t = true)]
    isremote: bool,

    #[arg(long, default_value = "https://api.deepseek.com/v1")]
    api_base: String,
    #[arg(long)]
    strict_coverage: bool,

    #[arg(long, default_value_t = 1)]
    run_tests_multiple_times: i32,

    #[arg(long)]
    use_report_coverage_feature_flag: bool,
}

use crate::unitest_agent_test_generator::TestYaml;


 fn main() {
    let args = Args::parse();
    env::set_var("DEEPSEEK_API_KEY", "sk-1440a8d97ed1482dae7cc633dd8275c7");
    let report_filepath = args.report_filepath.clone();
    let agent_unitest = UnitestAgent::new(
        args.source_file_path,
        args.test_file_path,
        args.test_file_output_path,
        args.code_coverage_report_path, 
        args.test_command,
        args.test_command_dir,
        args.included_files,
        args.coverage_type,
        args.report_filepath,
        args.desired_coverage,
        args.max_iterations,
        args.additional_instructions,
        args.model,
        args.isremote,
        args.api_base,
        args.strict_coverage,
        args.run_tests_multiple_times,
        args.use_report_coverage_feature_flag
    );
    let mut agent_unitest: UnitestAgent = agent_unitest;
    agent_unitest.run();
    // let content = std::fs::read_to_string("src/tst.toml").unwrap();
    // let yaml_struct: TestYaml = match serde_yaml::from_str(&content) {
    //     Ok(tests) => {
    //         println!("====tests:==== {:?}", tests);
    //         tests
    //     }
    //     Err(e) => {
    //         // error!("Error parsing YAML response: {}", e);
    //         println!("====Error parsing YAML response: {}", e);
    //         panic!("Error parsing YAML response: {}", e);
    //     }
    // };

    
}

