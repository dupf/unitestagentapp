use serde_json::{json, Value};
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
// use wandb::TraceMeta;

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
        match model {
            "deepseek-coder" => {
                open_ai = OpenAI::default()
                    .with_config(
                        OpenAIConfig::default()
                            .with_api_base(api_base.to_string())
                            .with_api_key(
                                "sk-1440a8d97ed1482dae7cc633dd8275c7"
                            ),
                    )
                    .with_model(model.to_string());  
            }
            _ => {
                panic!("Model not supported");
            }
        };

        Self {
            model: model.to_string(),
            api_base: api_base.to_string(),
            // client: Client::new(),
            open_ai: open_ai,
        }
    }

    pub async fn call_remotedeepseek(
        &self,
        prompt: &std::collections::HashMap<String, String>,
        max_tokens: usize,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Create the OpenAI instance with DeepSeek configuration
        let prompt_info: langchain_rust::prompt::MessageFormatterStruct = message_formatter![
            fmt_message!(Message::new_system_message(prompt["system"].clone())),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            )))
        ];

        //We can now combine these into a simple LLM chain:
        let chain = LLMChainBuilder::new()
            .prompt(prompt_info)
            .llm(self.open_ai.clone())
            .build()
            .unwrap();

        match chain
            .invoke(prompt_args! {
            "input" => prompt["user"].clone()

            })
            .await
        {
            Ok(result) => {
                // println!("Result: {:?}", result);
                Ok(result)
            }
            Err(e) => {
                panic!("Error invoking LLMChain: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn call_remotedeepseekstream(
        &self,
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

        let mut output = String::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(value) => {
                    // 使用 content() 方法获取实际内容
                    value.to_stdout().unwrap();
                    output.push_str(&value.content);
                }

                Err(e) => return Err(Box::new(e)),
            }
        }
        Ok(output)
    }
}
