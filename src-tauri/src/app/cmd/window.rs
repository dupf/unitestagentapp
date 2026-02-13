use std::fs;
use std::path::Path;
use tauri::Manager;
#[tauri::command]
pub fn new_window(app: tauri::AppHandle, label: String, title: String, url: String) {
    // log::info!("> send message: length: option: {:?} {},{}", app, label, title);

    let win = app.get_window(&label);
    if win.is_none() {
        tauri::async_runtime::spawn(async move {
            tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App(url.parse().unwrap()))
                .title(title)
                .inner_size(700.0, 550.0)
                .resizable(true)
                .build()
                .unwrap();
        });
    } else if let Some(v) = win {
        if !v.is_visible().unwrap() {
            v.show().unwrap();
        }
        v.eval("window.location.reload()").unwrap();
        v.set_focus().unwrap();
    }
}

#[tauri::command]
pub fn window_reload(app: tauri::AppHandle, label: &str) {
    app.app_handle()
        .get_window(label)
        .unwrap()
        .eval("window.location.reload()")
        .unwrap();
}

#[tauri::command]
pub fn download_report(srcpath: String, destpath: String) -> Result<(), String> {
    log::info!("> send message: length: option:  {},{}", srcpath, destpath);

    log::info!("download report from ======= to =======");
    // 检查源文件是否存在
    if !Path::new(&srcpath).exists() {
        log::error!("源文件不存在");
        return Err("源文件不存在".to_string());
    }

    // 执行文件复制操作
    fs::copy(&srcpath, &destpath).map_err(|e| e.to_string())?;
    Ok(())
}
