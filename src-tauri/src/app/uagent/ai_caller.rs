use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
// use wandb::TraceMeta;
use tauri::{AppHandle, Manager};

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

use futures::StreamExt; // Add this line to bring StreamExt into scope

use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_placeholder, fmt_template,
    language_models::llm::LLM,
    llm::openai::{OpenAI, OpenAIModel},
    llm::OpenAIConfig,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};  

pub struct AICaller {
    model: String,
    api_base: String,
    // client: Client,
    open_ai: OpenAI<OpenAIConfig>,
}

fn resource_path(relative_path: &str) -> String {
    // Note: This is a simplified version. Rust doesn't have a direct PyInstaller equivalent
    let base_path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    format!("{}/{}", base_path, relative_path)
}

impl AICaller {
    pub fn new(model: &str, api_base: &str) -> Self {
        let open_ai: OpenAI<OpenAIConfig>;
        log::info!("Model: {}", model);
        log::info!("API Base: {}", api_base);
        match model {
            "deepseek"
            => {
                let  api_base = "https://api.deepseek.com/v1";
                let  model = "deepseek-reasoner";
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base(api_base.to_string())
                            .with_api_key(
                                "sk-1440a8d97ed1482dae7cc633dd8275c7"
                                // "sk-nfvfgidnunwkdpcklscuwcrkpuofywmtbrhgflsjztcensho"
                            ),
                    )
                    .with_model(model.to_string());  
            }
            "deepseek-tc" => {
                let model = "deepseek-r1";
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base("https://api.lkeap.cloud.tencent.com/v1".to_string())
                            .with_api_key(
                                "sk-7NhaHwnlZkX2kN9jjxDfP9H9HeY8xnPETY5318uhB3J0qAYY"
                            ),
                    )
                    .with_model(model.to_string());  
            }
            "deepseek-fast"=> {
                let model = "deepseek-ai/DeepSeek-V3";
                // let api_base = "https://api.deepseek.com/v1";
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base("https://api.siliconflow.cn/v1".to_string())
                            .with_api_key(
                                "sk-nfvfgidnunwkdpcklscuwcrkpuofywmtbrhgflsjztcensho"
                            ),
                    )
                    .with_model(model.to_string());  
            }
            "deepseek-gpt"  => {
                let model ="gpt-4o";
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base("https://api.openai.com/v1".to_string())
                            .with_api_key(
                            "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"
                            ),
                    )
                    .with_model(model.to_string());  
            }
            "deepseek-tester" => {
                let model   = "claude-3-7-sonnet-thinking";
                let api_base = "https://api.mjdjourney.cn/v1";
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base(api_base.to_string())
                            .with_api_key(
                                "sk-99h6gVR0Xrz96uyO991d81661cE742C4990d5f84B0B3B07e"
                            // "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"
                            ),
                    )
                    .with_model(model.to_string());  

            }
            "deepseek-tester_v1" => {

                let model    =  "deepseek-reasoner";
                let api_base: &str = "https://api.mjdjourney.cn/v1";
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base(api_base.to_string())
                            .with_api_key(
                                
                            ),
                    )
                    .with_model(model.to_string());  
            }
            _ => {
                panic!("Model not supported{}", model);
            }
        };
        Self {
            model: model.to_string(),
            api_base: api_base.to_string(),
            open_ai: open_ai,
        }
    }

    

    pub async fn call_remotedeepseekstream(
        &self,
        handle: &AppHandle,
        id: u64,
        prompt: &std::collections::HashMap<String, String>,
        max_tokens: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
        
        // println!("==call_remotedeepseekstream:== {}", prompt["user"].clone());
        // Create the OpenAI instance with DeepSeek configuration
        let prompt_info: langchain_rust::prompt::MessageFormatterStruct = message_formatter![
            fmt_message!(Message::new_system_message(prompt["system"].clone())),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            )))
        ];
        //We can now combine these into a simple LLM chain:
        let chain: langchain_rust::chain::LLMChain = LLMChainBuilder::new()
            .prompt(prompt_info)
            .llm(self.open_ai.clone())
            .build()
            .unwrap();

        let mut stream = chain
            .stream(prompt_args! {
            "input" => prompt["user"].clone(),
                })
            .await
            .unwrap();

        let finish_reason: String = "finish".to_string();
        let role: String = "user".to_string();

        let mut output = String::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(value) => {
                    // 使用 content() 方法获取实际内容
                    value.to_stdout().unwrap();
                    output.push_str(&value.content);

                    let progress: ProgressPayload = ProgressPayload {
                        id,
                        detail: value.content.clone(), 
                        role: role.clone(),
                        finish_reason: finish_reason.clone(), // Clone finish_reason to avoid moving it
                    };
                    progress.emit_progress(&handle);
                
                }

                Err(e) => return Err(Box::new(e)),
            }
        
            
        }
        Ok(output)
    }
}
