use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct LLMCaller {
    model: String,
    api_base: String,
}

impl LLMCaller {
    pub fn new(model: String, api_base: String) -> Self {
        LLMCaller { model, api_base }
    }

    pub async fn call_model(&self, prompt: &HashMap<String, String>, max_tokens: usize) -> Result<(String, usize, usize), Box<dyn Error>> {
        if !prompt.contains_key("system") || !prompt.contains_key("user") {
            return Err("The prompt dictionary must contain 'system' and 'user' keys.".into());
        }

        let messages = if prompt["system"].is_empty() {
            vec![json!({"role": "user", "content": prompt["user"]})]
        } else {
            vec![
                json!({"role": "system", "content": prompt["system"]}),
                json!({"role": "user", "content": prompt["user"]}),
            ]
        };

        let mut completion_params = json!({
            "model": self.model,
            "messages": messages,
            "max_tokens": max_tokens,
            "stream": true,
            "temperature": 0.2,
        });

        if self.model.contains("ollama") || self.model.contains("huggingface") || self.model.starts_with("openai/") {
            completion_params["api_base"] = json!(self.api_base);
        }

        let client = Client::new();
        // let mut url = "https://api.openai.com/v1/chat/completions".to_string();
        
        let mut url = "https://api.openai.com/v1/chat/completions".to_string();

        
        if !self.api_base.is_empty() {
            url = self.api_base.clone();
        }

        let response = client.post(&url)
            .json(&completion_params)
            .send()
            .await?;

        println!("Streaming results from LLM model...");
        let mut chunks = Vec::new();
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            print!("{}", chunk_str);
            chunks.push(chunk_str.to_string());
            thread::sleep(Duration::from_millis(10));
        }
        println!();

        // Note: The stream_chunk_builder functionality is not directly translatable to Rust
        // You would need to implement a custom function to combine the chunks into a single response

        let model_response = self.combine_chunks(&chunks)?;


        Ok((
            model_response["choices"][0]["message"]["content"].as_str().unwrap().to_string(),
            model_response["usage"]["prompt_tokens"].as_u64().unwrap() as usize,
            model_response["usage"]["completion_tokens"].as_u64().unwrap() as usize,
        ))
    }

    fn combine_chunks(&self, chunks: &[String]) -> Result<Value, Box<dyn Error>> {
        // Implement custom logic to combine chunks into a single response
        // This is a simplified version and may need to be adjusted based on the actual response format
        let combined = chunks.join("");
        let parsed: Value = serde_json::from_str(&combined)?;
        Ok(parsed)
    }
}