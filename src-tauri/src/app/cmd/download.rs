use std::{fs::{self}, path::PathBuf};
use std::path::Path;

use log::info;

#[tauri::command]
pub async fn download_img(name: String, blob: Vec<u8>) -> Result<String, String> {
  info!("save image {}", name);
  // let win = app.app_handle().get_window("core");
  let download_path = tauri::api::path::download_dir().unwrap().join(PathBuf::from(name));
  fs::write(&download_path, blob).unwrap();
  
  Ok(download_path.display().to_string())
}


#[tauri::command]
pub async fn download_report(src_path: String, dest_path: String) -> Result<(), String> {
  println!("download report from ======={} to ======={}", src_path, dest_path); 
  // info!("download report from ======={} to ======={}", src_path, dest_path); 
  // 检查源文件是否存在
  if !Path::new(&src_path).exists() {
      return Err("源文件不存在".to_string());
  }
  info!("download report from ======={} to ======={}", src_path, dest_path); 

  // 执行文件复制操作 
  fs::copy(&src_path, &dest_path).map_err(|e| e.to_string())?;
  Ok(())
}