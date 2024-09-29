use std::{fs::{self}, path::PathBuf};
use std::path::Path;
use tauri::{AppHandle};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
// use markup5ever::rcdom::RcDom;
use docx_rs::{Docx, Paragraph, Run};
use html2text::from_read;
use log::{error, info};
// use std::fs::File;
// use std::io::Write;
use std::fs::File;
use std::io::{self, Read, Write};
// use tauri::api::path::resource_dir;
// Define a struct to hold the parsed test case data



#[tauri::command]
pub async fn convert_html_to_word(apphandle: tauri::AppHandle, srcpath: String, destpath: String) -> Result<(), String> {
    // 检查源文件是否存在
//     let package_info = tauri::api::package_info(&apphandle);
//  let env_info = tauri::api::env(&apphandle);
  
//     let resource_dir_path = resource_dir(&package_info, &env_info);
//     let srcpath2 = resource_dir_path.unwrap().join(srcpath);

    let srcpath2 = apphandle
        .path_resolver()
        .resolve_resource(srcpath)
        .expect("failed to resolve resource");
  // } else {
  //   println!("Resource directory not found");
  // }
    info!("srcpath2: {}", srcpath2.to_str().unwrap());
    let mut file = File::open(srcpath2).expect("Failed to open HTML file");
    let mut html_content = String::new();
    file.read_to_string(&mut html_content)
        .expect("Failed to read HTML file");
 

    // Convert HTML table to plain text (you can also extract it differently depending on the complexity of the HTML)
    let table_text = from_read(html_content.as_bytes(), 80);
    // Create a new Word document
    let file = std::fs::File::create(&destpath).expect("Unable to create file");


    // Start writing into the Word document
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Table extracted from HTML")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&table_text)));

    // Save the document
    doc.build().pack(file).expect("Unable to write to file");
    println!("Word document created successfully.");


    Ok(())

}

#[tauri::command]
pub async fn download_img(handle: AppHandle,name: String, blob: Vec<u8>) -> Result<String, String> {
  info!("save image {}", name);
  // let win = app.app_handle().get_window("core");
  let download_path = tauri::api::path::download_dir().unwrap().join(PathBuf::from(name));
  fs::write(&download_path, blob).unwrap();
  Ok(download_path.display().to_string())
}


 