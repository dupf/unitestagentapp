use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct TestGenerationPrompt {
    pub system: String,
    pub user: String,
}

#[derive(Deserialize, Debug)]
pub struct PromptConfig {   

    
    // 使用 HashMap 来存储所有的 prompts
    #[serde(flatten)]
    pub prompts: HashMap<String, TestGenerationPrompt>,
}

impl PromptConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: PromptConfig = toml::from_str(&content)?;
        Ok(config)
    }

    // 根据 section 名称获取对应的 prompt
    pub fn get_prompt(&self, section: &str) -> Option<&TestGenerationPrompt> {
        self.prompts.get(section)
    }

    pub fn get_system_prompt(&self, section: &str) -> Option<&str> {
        self.get_prompt(section).map(|p| p.system.as_str())
    }

    pub fn get_user_prompt(&self, section: &str) -> Option<&str> {
        self.get_prompt(section).map(|p| p.user.as_str())
    }
}
