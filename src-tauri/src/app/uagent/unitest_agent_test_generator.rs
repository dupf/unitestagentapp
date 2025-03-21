use log::{error, info};
use rocket::build;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
// use crate::llm_caller::LLMCaller;
// use crate::llm_request::LLMRequest;
use tauri::{AppHandle, Manager};

use crate::app::uagent::{
    ai_caller::AICaller, file_preprocessor::FilePreprocessor, prompt_builder::PromptBuilder,
};

pub struct UnitTestAgentTestGenerator {
    source_file_path: PathBuf,
    test_file_path: PathBuf,
    code_coverage_report_path: PathBuf,
    test_command: String,
    test_command_dir: PathBuf,
    included_files: String,
    coverage_type: String,
    desired_coverage: i32,
    additional_instructions: String,
    language: String,
    use_report_coverage_feature_flag: bool,
    last_coverage_percentages: HashMap<String, f64>,
    isremote: bool,
    // llm_caller: Box<dyn LLMInterface>, // Using trait object for polymorphism
    llm_caller: AICaller,
    // logger: Logger,
    preprocessor: FilePreprocessor,
    failed_test_runs: Vec<HashMap<String, String>>,
    total_input_token_count: i32,
    total_output_token_count: i32,
    current_coverage: f64,
    prompt: String,
    prompt_builder: PromptBuilder,
}

// #[derive(Debug, serde::Deserialize)]
// pub struct TestYaml {
//     language: String,
//     existing_test_function_signature: String,
//     new_tests: Vec<TestDetails>,
// }

#[derive(Debug, serde::Deserialize)]
pub struct TestYaml {
    language: String,
    total_tests: String,
    new_tests: Vec<TestDetails>,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestDetails {
    #[serde(rename = "测试用例编号")]
    pub test_number: Option<String>,
    #[serde(rename = "测试技术")]
    pub test_technique: Option<String>,
    #[serde(rename = "测试用例描述")]
    pub test_description: Option<String>,
    #[serde(rename = "测试代码")]
    pub test_code: Option<String>,
    #[serde(rename = "全局变量")]
    pub global_variables: Option<String>,
    #[serde(rename = "初始化代码")]
    pub initialization_code: Option<String>,
    #[serde(rename = "桩函数")]
    pub stub_functions: Option<String>,
    #[serde(rename = "输入")]
    pub input: Option<String>,
    #[serde(rename = "预期输出")]
    pub expected_output: Option<String>,
    #[serde(rename = "实际输出")]
    pub actual_output: Option<String>,
    #[serde(rename = "结论")]
    pub conclusion: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TestDetailsEn {
    #[serde(rename = "test_number")]
    pub test_number: Option<String>,
    #[serde(rename = "test_technique")]
    pub test_technique: Option<String>,
    #[serde(rename = "test_description")]
    pub test_description: Option<String>,
    #[serde(rename = "test_code")]
    pub test_code: Option<String>,
    #[serde(rename = "global_variables")]
    pub global_variables: Option<String>,
    #[serde(rename = "initialization_code")]
    pub initialization_code: Option<String>,
    #[serde(rename = "stub_functions")]
    pub stub_functions: Option<String>,
    #[serde(rename = "input")]
    pub input: Option<String>,
    #[serde(rename = "expected_output")]
    pub expected_output: Option<String>,
    #[serde(rename = "actual_output")]
    pub actual_output: Option<String>,
    #[serde(rename = "conclusion")]
    pub conclusion: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeIssue {
    #[serde(rename = "问题编号")]
    pub issue_id: Option<String>,
    #[serde(rename = "类别")]
    pub category: Option<String>,
    #[serde(rename = "严重性")]
    pub severity: Option<String>,
    #[serde(rename = "描述")]
    pub description: Option<String>,
    #[serde(rename = "位置")]
    pub location: Option<String>,
    #[serde(rename = "影响")]
    pub impact: Option<String>,
    #[serde(rename = "建议")]
    pub recommendation: Option<String>,
    #[serde(rename = "最佳实践")]
    pub best_practice: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeIssueEn {
    #[serde(rename = "issue_id")]
    pub issue_id: Option<String>,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "severity")]
    pub severity: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "location")]
    pub location: Option<String>,
    #[serde(rename = "impact")]
    pub impact: Option<String>,
    #[serde(rename = "recommendation")]
    pub recommendation: Option<String>,
    #[serde(rename = "best_practice")]
    pub best_practice: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeAnalysisResult {
    #[serde(rename = "语言")]
    pub language: String,

    #[serde(rename = "问题总数")]
    pub total_issues: i32,

    #[serde(rename = "代码规范问题")]
    pub coding_standard_issues: Vec<CodeIssue>,

    #[serde(rename = "性能问题")]
    pub performance_issues: Vec<CodeIssue>,

    #[serde(rename = "安全漏洞")]
    pub security_vulnerabilities: Vec<CodeIssue>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeAnalysisResultEn {
    #[serde(rename = "language")]
    pub language: String,

    #[serde(rename = "total_issues")]
    pub total_issues: i32,

    #[serde(rename = "coding_standard_issues")]
    pub coding_standard_issues: Vec<CodeIssueEn>,

    #[serde(rename = "performance_issues")]
    pub performance_issues: Vec<CodeIssueEn>,

    #[serde(rename = "security_vulnerabilities")]
    pub security_vulnerabilities: Vec<CodeIssueEn>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ProgressPayload {
    pub id: u64,
    pub detail: String,
    pub role: String,
    pub finish_reason: String,
}

impl ProgressPayload {
    pub fn emit_progress(&self, handle: &AppHandle) {
        handle.emit_all("CHAT_FETCHEING_PROGRESS", &self).ok();
    }
}
impl TestDetails {
    pub fn to_english(&self) -> TestDetailsEn {
        TestDetailsEn {
            test_number: self.test_number.clone(),
            test_technique: self.test_technique.clone(),
            test_description: self.test_description.clone(),
            test_code: self.test_code.clone(),
            global_variables: self.global_variables.clone(),
            initialization_code: self.initialization_code.clone(),
            stub_functions: self.stub_functions.clone(),
            input: self.input.clone(),
            expected_output: self.expected_output.clone(),
            actual_output: self.actual_output.clone(),
            conclusion: self.conclusion.clone(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct FailDetails {
    status: String,
    reason: String,
    exit_code: Option<i32>,
    stderr: String,
    stdout: String,
    test: String,
}

impl UnitTestAgentTestGenerator {
    pub fn new(
        source_file_path: &str,
        test_file_path: &str,
        code_coverage_report_path: &str,
        test_command: &str,
        test_command_dir: Option<&str>,
        included_files: Option<&Vec<String>>,
        coverage_type: Option<&str>,
        desired_coverage: Option<i32>,
        llm_model: &str,
        api_base: &str,
        additional_instructions: &str,
        use_report_coverage_feature_flag: bool,
        isremote: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let test_command_dir = test_command_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| env::current_dir().unwrap());
        let included_files_content = Self::get_included_files(included_files.cloned());
        let language = Self::get_code_language(&test_file_path);
        let llm_caller: AICaller = AICaller::new(llm_model, api_base);
        // let preprocessor: <FilePreprocessor as Try>::Output = FilePreprocessor::new(&test_file_path)?;
        let preprocessor = FilePreprocessor::new(test_file_path.to_string());

        let prompt_builder_orign: PromptBuilder = PromptBuilder::new(
            &source_file_path, // Now we can use the original String
            &preprocessor.path_to_file,
            &code_coverage_report_path, // Now we can use the original String
            "included_files",
            &additional_instructions,
            "failed_test_runs",
            &language,
        );

        let mut generator: UnitTestAgentTestGenerator = Self {
            source_file_path: PathBuf::from(source_file_path),
            test_file_path: PathBuf::from("./test_file_path.py"),
            code_coverage_report_path: PathBuf::from(code_coverage_report_path),
            test_command: test_command.to_string(),
            test_command_dir,
            included_files: included_files_content,
            coverage_type: "cobertura".to_string(),
            desired_coverage: desired_coverage.unwrap_or(90),
            additional_instructions: additional_instructions.to_string(),
            language: String::from("C"),
            use_report_coverage_feature_flag,
            last_coverage_percentages: HashMap::new(),
            isremote,
            llm_caller,
            preprocessor,
            failed_test_runs: Vec::new(),
            total_input_token_count: 0,
            total_output_token_count: 0,
            current_coverage: 0.0,
            prompt: String::new(),
            prompt_builder: prompt_builder_orign,
        };

        Ok(generator)
    }

    fn get_code_language(file_path: &str) -> String {
        // let settings: ! = get_settings()?;
        // let mut extension_to_language = HashMap::new();
        // for (language, extensions) in settings.language_extension_map_org {
        //     for ext in extensions {
        //         extension_to_language.insert(ext, language.clone());
        //     }
        // }
        // let extension = self.source_file_path
        //     .extension()
        //     .and_then(|ext| ext.to_str())
        //     .map(|s| format!(".{}", s))
        //     .unwrap_or_default();
        // Ok(extension_to_language
        //     .get(&extension)
        //     .map(|s| s.to_lowercase())
        //     .unwrap_or_else(|| String::from("unknown")))
        // Ok(String::from("C"))
        String::from("C")
    }

    fn get_included_files(included_files: Option<Vec<String>>) -> String {
        if let Some(files) = included_files {
            let mut content = Vec::new();

            let mut file_names = Vec::new();
            for file_path in files {
                match fs::read_to_string(&file_path) {
                    Ok(file_content) => {
                        content.push(file_content);
                        file_names.push(file_path);
                    }
                    Err(e) => eprintln!(
                        "included_files not Found && Error reading file {}: {}",
                        file_path, e
                    ),
                }
            }
            if !content.is_empty() {
                return content
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        format!(
                            "file_path: `{}`\ncontent:\n```\n{}\n```\n",
                            file_names[i], c
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string();
            }
        }
        String::new()
        // content
    }

    async fn call_remoteinfo(
        &mut self,
        handle: AppHandle,
        id: u64,
        prompt: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // match self.llm_caller.call_remotedeepseek(prompt, 4096).await
        match self
            .llm_caller
            .call_remotedeepseekstream(&handle, id, prompt, 40960)
            .await
        {
            Ok(response) => {
                // Handle potential UTF-8 encoding issues
                let cleaned_response = response
                    .chars()
                    .filter(|c| c.is_ascii() || c.is_alphabetic())
                    .collect::<String>();

                Ok(cleaned_response)
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        }
    }
    pub async fn initial_test_suite_analysis(
        &mut self,
        handle: AppHandle,
        id: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut test_headers_indentation: Option<String> = None;
        let allowed_attempts = 3;
        let mut counter_attempts = 0;

        while test_headers_indentation.is_none() && counter_attempts < allowed_attempts {
            let prompt_headers_indentation = self.prompt_builder.build_prompt_custom(
                "prompts/analyze_suite_test_headers_indentation.toml",
                "analyze_suite_test_headers_indentation",
            )?;

            let mut prompt_map: HashMap<String, String> = std::collections::HashMap::new();
            prompt_map.insert("system".to_string(), prompt_headers_indentation.0);
            prompt_map.insert("user".to_string(), prompt_headers_indentation.1);

            let response: String = self
                .call_remoteinfo(handle.clone(), id, &prompt_map)
                .await?;


            let tests_dict: Result<HashMap<String, i32>, serde_yaml::Error> =
                match serde_yaml::from_str(&response) {
                    Ok(dict) => Ok(dict),
                    Err(e) => {
                        error!("YAML parsing error: {}. Content: {}", e, response);
                        Err(e)
                    }
                };

            let tests_dict: HashMap<String, i32> = tests_dict?;

            test_headers_indentation = tests_dict
                .get("test_headers_indentation")
                .map(|v: &i32| v.to_string());

            counter_attempts += 1;
        }
        let test_headers_indentation = test_headers_indentation.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to analyze the test headers indentation",
            )
        })?;

        // Get insert line numbers
        let mut insert_lines: Option<(i32, i32)> = None;
        let mut counter_attempts = 0;
        let tests_after: Option<i32> = None;
        let imports_after: Option<i32> = None;
        while insert_lines.is_none() && counter_attempts < allowed_attempts {
            let prompt_test_insert_line = self.prompt_builder.build_prompt_custom(
                "prompts/analyze_suite_test_insert_line.toml",
                "analyze_suite_test_insert_line",
            )?;

            let mut prompt_map = std::collections::HashMap::new();
            prompt_map.insert("system".to_string(), prompt_test_insert_line.0);
            prompt_map.insert("user".to_string(), prompt_test_insert_line.1);

            let response = self
                .call_remoteinfo(handle.clone(), id, &prompt_map)
                .await?;

            let yaml_content = if let Some(content) = response.split("```yaml").nth(1) {
                if let Some(yaml) = content.split("```").next() {
                    yaml.trim()
                } else {
                    &response
                }
            } else {
                &response
            };

            let tests_dict: Result<HashMap<String, i32>, serde_yaml::Error> =
                match serde_yaml::from_str(&yaml_content) {
                    Ok(dict) => Ok(dict),
                    Err(e) => {
                        error!("YAML parsing error: {}. Content: {}", e, yaml_content);
                        Err(e)
                    }
                };

            let yaml_content2: String = format!("{}", yaml_content);

            let tests_dict: HashMap<String, i32> = tests_dict?;

            let tests_after = tests_dict.get("relevant_line_number_to_insert_tests_after");
            let imports_after = tests_dict.get("relevant_line_number_to_insert_imports_after");

            if let (Some(&tests), Some(&imports)) = (tests_after, imports_after) {
                insert_lines = Some((tests, imports));
            }
            counter_attempts += 1;
        }
        let test_headers_indentation = Some(test_headers_indentation);
        let relevant_line_number_to_insert_tests_after = tests_after;
        let relevant_line_number_to_insert_imports_after = imports_after;

        Ok(())
    }

    pub async fn generate_tests(
        &mut self,
        handle: AppHandle,
        id: u64,
        max_tokens: usize,
        dry_run: bool,
    ) -> Result<Vec<TestDetails>, Box<dyn std::error::Error>> {
        // self.prompt = self.build_prompt()?;
        let response = if dry_run {
            String::from("```def test_something():\n    pass```\n```def test_something_else():\n    pass```\n```def test_something_different():\n    pass```")
        } else {
            let prompt_tests = self.prompt_builder.build_prompt_custom(
                "prompts/test_generation_prompt.toml",
                "test_generation_prompt_htzr_cn",
            )?;
            let mut prompt_map = std::collections::HashMap::new();
            prompt_map.insert("system".to_string(), prompt_tests.0);
            prompt_map.insert("user".to_string(), prompt_tests.1);

            self.call_remoteinfo(handle.clone(), id, &prompt_map)
                .await?
        };
        // println!("response==========: {:?}", response);
        let yaml_content = if let Some(content) = response.split("```yaml").nth(1) {
            if let Some(yaml) = content.split("```").next() {
                yaml.trim()
            } else {
                &response
            }
        } else {
            &response
        };
        // Parse YAML response into TestDetails
        let yaml_content2: String = format!("{}", yaml_content);
        // println!("yaml_content2==========: {:?}", yaml_content2);
        let yaml_struct: TestYaml = match serde_yaml::from_str(&yaml_content2) {
            Ok(tests) => tests,
            Err(e) => TestYaml {
                language: "".to_string(),
                total_tests: "".to_string(),
                new_tests: vec![],
            },
        };
        Ok(yaml_struct.new_tests)
    }

    pub async fn generate_static_sec(
        &mut self,
        handle: AppHandle,
        id: u64,
        max_tokens: usize,
        dry_run: bool,
    ) -> Result<CodeAnalysisResult, Box<dyn std::error::Error>> {
        // self.prompt = self.build_prompt()?;
        let response = if dry_run {
            String::from("```def test_something():\n    pass```\n```def test_something_else():\n    pass```\n```def test_something_different():\n    pass```")
        } else {
            let prompt_tests = self.prompt_builder.build_prompt_custom(
                "prompts/static_code_analysis_cn.toml",
                "static_code_analysis_prompt_cn",
            )?;
            let mut prompt_map = std::collections::HashMap::new();
            prompt_map.insert("system".to_string(), prompt_tests.0);
            prompt_map.insert("user".to_string(), prompt_tests.1);

            self.call_remoteinfo(handle.clone(), id, &prompt_map)
                .await?
        };

        let yaml_content = if let Some(content) = response.split("```yaml").nth(1) {
            if let Some(yaml) = content.split("```").next() {
                yaml.trim()
            } else {
                &response
            }
        } else {
            &response
        };
        // Parse YAML response into TestDetails
        let yaml_content2: String = format!("{}", yaml_content);


        let code_analysis_result: CodeAnalysisResult = match serde_yaml::from_str(&yaml_content2) {
            Ok(tests) => tests,
            Err(e) => CodeAnalysisResult {
                language: "".to_string(),
                total_issues: 0,
                coding_standard_issues: vec![],
                performance_issues: vec![],
                security_vulnerabilities: vec![],
            }
        };

        Ok(code_analysis_result)
    }
}
