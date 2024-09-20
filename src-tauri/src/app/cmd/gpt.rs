use tauri::{AppHandle, Manager};
use reqwest::{self, Url};
use eventsource_stream::{Eventsource, EventStreamError};
use serde_json::{json, Value};
use serde::{ser::Serializer, Serialize, Deserialize};
use futures::{TryStreamExt};
use std::{ time::Duration, env::consts::OS };
use log::{error, info};
use std::process::{Command, Stdio};
use futures::stream::StreamExt;
use tokio_util::io::StreamReader;
use tokio::io::AsyncBufReadExt;

type Result<T> = std::result::Result<T, Error>;

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
    Custom{code: u16, msg: String}
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
    pub content: String
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

#[tauri::command]
pub async fn fetch_chat_api(
    handle: AppHandle,
    id: u64,
    messages: Vec<Message>,
    option: FetchOption,
) -> Result<u64> {
    
    let data = json!({
        "model": option.model,
        "messages": messages,
        "temperature": option.temperature,
        "stream": true,
    });
    log::info!("> send message: length: {}, option: {:?},", messages.len(), option);
    let proxy_str = option.proxy.unwrap_or(String::from(""));

    let client : reqwest::Client = {
        log::info!("proxy is: {}", proxy_str);
        let mut client_builder = reqwest::Client::builder();
        if proxy_str.len()>0 {
            let proxy = reqwest::Proxy::all(proxy_str).unwrap();
            client_builder = client_builder.proxy(proxy);
        }
        client_builder.build().unwrap()
    };

    let api_url = Url::parse(&option.host).unwrap().join("/v1/chat/completions").unwrap().as_str().to_owned();

    let res = client.post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", option.apiKey))
        .header(reqwest::header::USER_AGENT, format!("ChatGPT-Tauri ({})", OS))
        .timeout(Duration::from_secs(600))
        .body(data.to_string())
        .send()
        .await?;
    info!("> receive message: {}", id);
    
    let status_code = res.status().as_u16();
    if status_code != 200 {
        let error_msg = res.text().await?;
        log::error!("{}", error_msg);
        return Err(Error::Custom {code: status_code, msg:String::from(error_msg)})
    }

    let mut stream = res.bytes_stream().eventsource();
    while let Some(chunk) = stream.try_next().await? {
        let chunk = chunk.data;
        if chunk == "[DONE]" {
            return Ok(id)
        } else {
            let object:Value = serde_json::from_str(&chunk)?;
            let delta: &Value = &object["choices"][0]["delta"];
            let content = String::from(delta["content"].as_str().unwrap_or(""));
            let role = String::from(delta["role"].as_str().unwrap_or(""));
            let finish_reason = String::from(object["finish_reason"].as_str().unwrap_or(""));
            let progress = ProgressPayload {id, detail:content, role, finish_reason};
            progress.emit_progress(&handle);
        }
    }
    Ok(id)
}


#[tauri::command]
pub async fn fetch_unitest_api(
    handle: AppHandle,
    id: u64,
    messages: Vec<Message>,
    option: FetchOption,
) -> Result<u64> {

    let data = json!({
        "model": option.model,
        "messages": messages,
        "temperature": option.temperature,
        "stream": true,
    });
    // println!("messages: {}", messages);

    // log::info!("> send message: {:#?}, length: {}, option: {:?},", messages, messages.len(), option);
    
    let user_contents: Vec<String> = messages.iter()
        .filter(|message| message.role == "user")
        .map(|message| message.content.clone())
        .collect();

    log::info!("User contents: {:?}", user_contents);
    // Parse user_contents
    let parsed_contents: Vec<String> = user_contents.iter()
        .flat_map(|content| content.split('|').map(String::from).collect::<Vec<String>>())
        .collect();

    let args: Vec<&str> = user_contents.iter().map(AsRef::as_ref).collect();

    // log::info!("Parsed contents: {:?}", parsed_contents);
    // let proxy_str = option.proxy.unwrap_or(String::from(""));
    // let client : reqwest::Client = {
    //     log::info!("proxy is: {}", proxy_str);
    //     let mut client_builder = reqwest::Client::builder();
    //     if proxy_str.len()>0 {
    //         let proxy = reqwest::Proxy::all(proxy_str).unwrap();
    //         client_builder = client_builder.proxy(proxy);
    //     }
    //     client_builder.build().unwrap()
    // };
    // let api_url = Url::parse(&option.host).unwrap().join("/v1/chat/completions").unwrap().as_str().to_owned();
    // let res = client.post(api_url)
    //     .header("Content-Type", "application/json")
    //     .header("Authorization", format!("Bearer {}", option.apiKey))
    //     .header(reqwest::header::USER_AGENT, format!("ChatGPT-Tauri ({})", OS))
    //     .timeout(Duration::from_secs(600))
    //     .body(data.to_string())
    //     .send()
    //     .await?;
    // info!("> receive message: {}", id);
    // let status_code = res.status().as_u16();
    // if status_code != 200 {
    //     let error_msg = res.text().await?;
    //     log::error!("{}", error_msg);
    //     return Err(Error::Custom {code: status_code, msg:String::from(error_msg)})
    // }
    // let mut stream = res.bytes_stream().eventsource();
    // while let Some(chunk) = stream.try_next().await? {
    //     let chunk = chunk.data;
    //     if chunk == "[DONE]" {
    //         return Ok(id)
    //     } else {
    //         let object:Value = serde_json::from_str(&chunk)?;
    //         let delta: &Value = &object["choices"][0]["delta"];
    //         let content = String::from(delta["content"].as_str().unwrap_or(""));
    //         let role = String::from(delta["role"].as_str().unwrap_or(""));
    //         let finish_reason: String = String::from(object["finish_reason"].as_str().unwrap_or(""));
    //         info!("content: {} {}  {} {} {} ", content, delta["content"].as_str().unwrap_or(""), delta["role"].as_str().unwrap_or(""),
    //         finish_reason, object["finish_reason"].as_str().unwrap_or(""));
    //         let progress: ProgressPayload = ProgressPayload {id, detail:content, role, finish_reason};
    //         progress.emit_progress(&handle);
    //     }     //   " /Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/JZ20_TZB_CANC.c /Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/testJZ20_TZB_AllData.c")
    // }         // &["/Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/JZ20_TZB_CANC.c", "/Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/testJZ20_TZB_AllData.c"])

    info!("> receive message: {}", id);
    let finish_reason: String = "finish".to_string();
    let mut child = Command::new("/Users/mac/Documents/work/htzr/unitest_agent/uapp/run_unitest.sh" )
    .args( args)
    .stdout(Stdio::piped())
    .spawn()?;

    use std::io::{BufRead, BufReader};
    
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    let mut content = String::new();
    while let Some(line) = lines.next().transpose()? {
        println!("{}", line);
        content.push_str(&line);
        content.push('\n'); // Add newline character after each line

        let role: String = "user".to_string();
        let progress: ProgressPayload = ProgressPayload {
            id,
            detail: content.clone(), // Clone content to avoid moving it
            role,
            finish_reason: finish_reason.clone(), // Clone finish_reason to avoid moving it
        };
        progress.emit_progress(&handle);
    }

    Ok(id)
    }
