use eventsource_stream::{EventStreamError, Eventsource};
use futures::stream::StreamExt;
use futures::TryStreamExt;
use log::{error, info};
use reqwest::{self, Url};
use serde::{ser::Serializer, Deserialize, Serialize};
use serde_json::{json, Value};
use std::process::{Command, Stdio};
use std::{env::consts::OS, time::Duration};
use tauri::{AppHandle, Manager};
// use tokio_util::io::StreamReader;
use tokio::io::AsyncBufReadExt;
use tauri::api::path::resource_dir;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};

use tauri::Env;
use tauri::PackageInfo;
type Result<T> = std::result::Result<T, Error>;


use crate::app::uagent::unitest_agent::UnitestAgent;




#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Stream(#[from] EventStreamError<reqwest::Error>),
    #[error("Custom Error: (code: {code:?}, message: {msg:?})")]
    Custom { code: u16, msg: String },
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct FetchOption {
    pub proxy: Option<String>,
    pub host: String,
    pub apiKey: String,
    pub model: String,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct FetchUnitestOption {

    pub model: Option<String>,

}


#[tauri::command]
pub async fn fetch_unitest(
    handle: AppHandle,
    id: u64,
    messages: Vec<Message>,
    option: FetchUnitestOption,
) -> Result<u64> {
    
    log::info!(
        "> send message: {:#?}, length: {}, option: {:?},",
        messages,
        messages.len(),
        option.model
    );
    let user_contents: Vec<String> = messages
        .iter()
        .filter(|message| message.role == "user")
        .map(|message| message.content.clone())
        .collect();

    log::info!("User contents: {:?}", user_contents);
    // Parse user_contents
    let parsed_contents: Vec<String> = user_contents
        .iter()
        .flat_map(|content: &String| {
            content
                .split('|')
                .map(String::from)
                .collect::<Vec<String>>()
        }).collect();

    log::info!("> receive message: {}", id);
    
    let package_info: PackageInfo = handle.package_info().clone();
    let env_info: Env = handle.env().clone();

    let resource_dir_path = resource_dir(&package_info, &env_info);

    let os: &str = std::env::consts::OS;
    // println!("Current operating system: {}", os);
    let unitest_agent_path: PathBuf;
    let finish_reason: String = "finish".to_string();


    // let api_base: String = "https://ark.cn-beijing.volces.com/api/v3/chat/completions".to_string();
    let api_base: String = "https://api.siliconflow.cn/v1/chat/completions".to_string();
    // "https://api.deepseek.com/v1".to_string();
    // "https://api.deepseek.com/v1".to_string();

    let strict_coverage: bool = false;
    let run_tests_multiple_times: i32 = 1;
    let use_report_coverage_feature_flag: bool = false;
    
    let mut agent_unitest = UnitestAgent::new(
        parsed_contents[0].clone(),
        parsed_contents[1].clone(),
        parsed_contents[2].clone(),
        parsed_contents[3].clone(),
        parsed_contents[4].clone(),
        parsed_contents[5].clone(),
        Some(vec![parsed_contents[6].clone()]),
        parsed_contents[7].clone(),
        parsed_contents[8].clone(),
        parsed_contents[9].parse::<i32>().unwrap_or(0),
        parsed_contents[10].parse::<i32>().unwrap_or(0),
        parsed_contents[11].clone(),
        parsed_contents[12].clone(),
        parsed_contents[13].parse::<bool>().unwrap_or(false),
        api_base,
        strict_coverage,
        run_tests_multiple_times,
        use_report_coverage_feature_flag
    );
    agent_unitest.run(handle,id).await;


    Ok(id)
}
