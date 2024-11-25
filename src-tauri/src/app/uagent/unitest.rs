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
    println!("Current operating system: {}", os);
    let unitest_agent_path: PathBuf;
    let finish_reason: String = "finish".to_string();


    let api_base: String = "https://api.deepseek.com/v1".to_string();

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
    agent_unitest.run().await;

    // let args = [
    //         ("--source-file-path", &parsed_contents[0]),
    //         ("--test-file-path", &parsed_contents[1]),
    //         ("--test-file-output-path", &parsed_contents[2]),
    //         ("--code-coverage-report-path", &parsed_contents[3]),
    //         ("--test-command", &parsed_contents[4]),
    //         ("--test-command-dir", &parsed_contents[5]),
    //         ("--included-files", &parsed_contents[6]),
    //         ("--coverage-type", &parsed_contents[7]),
    //         ("--report-filepath", &parsed_contents[8]),
    //         ("--desired-coverage", &parsed_contents[9]),
    //         ("--max-iterations", &parsed_contents[10]),
    //         ("--additional-instructions", &parsed_contents[11]),
    //         ("--model", &parsed_contents[12]),
    //         ("--isremote", &parsed_contents[13]),
    //     ];


        // unitest_agent_path = PathBuf::from(&parsed_contents[5]).
        // join(&parsed_contents[4]);
        // log::info!("unitest_agent_path: {:?}", unitest_agent_path);
   
        // let mut child: Command = Command::new(unitest_agent_path);
        // log::info!("==========Command: child:==========");
        // for (arg, value) in args.iter() {
        //     if !value.is_empty() {
        //         child.arg(arg);
        //         child.arg(value);
        //     }
        // }
        // let mut child = child
        // .stdout(Stdio::piped())
        // .spawn()?;

    // println!("parsed_contents: {:?}", child);
    // let stdout = child.stdout.take().expect("Failed to capture stdout");
    // let reader = BufReader::new(stdout);
    // let mut lines: std::io::Lines<BufReader<std::process::ChildStdout>> = reader.lines();
    // let mut content: String = String::new();
    // while let Some(line) = lines.next().transpose()? {
    //     println!("{}", line);
    //     content.push_str(&line);
    //     content.push('\n'); // Add newline character after each line
    //     let role: String = "user".to_string();

    //     let progress: ProgressPayload = ProgressPayload {
    //         id,
    //         detail: content.clone(), // Clone content to avoid moving it
    //         role,
    //         finish_reason: finish_reason.clone(), // Clone finish_reason to avoid moving it
    //     };
    //     progress.emit_progress(&handle);
    // }





    Ok(id)
}
