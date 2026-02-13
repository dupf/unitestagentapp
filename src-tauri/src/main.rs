#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod utils;

use app::{builder, cmd, uagent};
use tauri::Manager;
use tauri_plugin_log::{
  fern::colors::{Color, ColoredLevelConfig},
  LogTarget,
};

// #[tauri::command]
// async fn generate_static_analysis_report(
//     handle: tauri::AppHandle,
//     analysis_id: u64,
// ) -> Result<String, String> {
//     // 从保存的数据中获取静态代码分析结果
//     let generator = app::uagent::unitest_agent_test_generator::UnitTestAgentTestGenerator::new(
//         "",
//         "",
//         "",
//         "",
//         "",
//         None,
//         None,
//         Some(""),
//         Some(0),
//         "deepseek-tester",
//         "",
//         false,
//         false,
//     ).map_err(|e| e.to_string())?;
    
//     // 生成静态代码分析报告
//     let code_analysis_result = generator.generate_static_sec(handle.clone(), analysis_id, 0, false).await
//         .map_err(|e| e.to_string())?;
    
//     // 生成报告
//     let report_path = app::uagent::report_generator::ReportGenerator::generate_static_analysis_report(
//         handle,
//         &code_analysis_result,
//     ).await
//         .map_err(|e| e.to_string())?;
    
//     Ok(report_path)
// }

fn main() {
  let mut log = tauri_plugin_log::Builder::default()
  .targets([
    LogTarget::Folder(utils::app_root()),
    LogTarget::Stdout,
    LogTarget::Webview,
  ])
  .level(log::LevelFilter::Debug);

  if cfg!(debug_assertions) {
    log = log.with_colors(ColoredLevelConfig {
      error: Color::Red,
      warn: Color::Yellow,
      debug: Color::Blue,
      info: Color::BrightGreen,
      trace: Color::Cyan,
    });
  }

  let mut builder = tauri::Builder::default()
  .plugin(log.build())
  .invoke_handler(tauri::generate_handler![
    cmd::gpt::fetch_chat_api,
    // cmd::download::download_report,
    cmd::gpt::fetch_unitest_api,
    cmd::download::download_img,
    cmd::download::convert_html_to_word,
    cmd::window::download_report,
    cmd::window::new_window,
    uagent::unitest::fetch_unitest,
    // generate_static_analysis_report,
  ])
  .setup(builder::setup);


  #[cfg(target_os = "macos")]
  {
    builder = builder.on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        let win = event.window().clone();
        if win.label() == "core" {
          tauri::AppHandle::hide(&event.window().app_handle()).unwrap();
        }else {
          cmd::window::window_reload(event.window().app_handle(), "core");
          event.window().close().unwrap();
        }
        api.prevent_close();
      }
      _ => {}
    })
  }
  
  builder.run(tauri::generate_context!())
  .expect("error while running tauri application");
}
