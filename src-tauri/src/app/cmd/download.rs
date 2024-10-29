use std::{fs::{self}, path::PathBuf};
use std::path::Path;
use tauri::{AppHandle};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
// use markup5ever::rcdom::RcDom;
// use docx_rs::{Docx, Paragraph, Run};
use html2text::from_read;
use log::{error, info};
// use std::fs::File;
// use std::io::Write;
use std::fs::File;
use std::io::{self, Read, Write};
// use tauri::api::path::resource_dir;
// Define a struct to hold the parsed test case data
use docx_rs::*;
use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Class};

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
    let mut html_content: String = String::new();
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
pub async fn c_html_to_word(apphandle: tauri::AppHandle, srcpath: String, destpath: String) -> Result<(), String> {
    // 检查源文件是否存在
//     let package_info = tauri::api::package_info(&apphandle);
//  let env_info = tauri::api::env(&apphandle);
//     let resource_dir_path = resource_dir(&package_info, &env_info);
//     let srcpath2 = resource_dir_path.unwrap().join(srcpath);
    let srcpath2 = apphandle
        .path_resolver()
        .resolve_resource(srcpath)
        .expect("failed to resolve resource");

    info!("srcpath2: {}", srcpath2.to_str().unwrap());
    let mut file = File::open(srcpath2).expect("Failed to open HTML file");
    let mut html_content: String = String::new();
    
    file.read_to_string(&mut html_content)
        .expect("Failed to read HTML file");

    let document = Document::from(html_content.as_str());
    // 创建一个新的 Word 文档
    let mut doc = Docx::new();
    let mut table = Table::new(Vec::new()); // Fix: Provide the required argument to the Table::new() function

    // 解析 HTML 表格的标题行
    if let Some(header_row) = document.find(Name("tr")).next() {
        let mut header_cells: Vec<_> = Vec::new();
        for header_cell in header_row.find(Name("th")) {
            let text = header_cell.text();
            header_cells.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(text))));
        }
        table = table.add_row(TableRow::new(header_cells));
    }
    
    for row in document.find(Name("tr")).skip(1) {
        let mut cells = Vec::new();
        for cell in row.find(Name("td")) {
            let text = cell.text();
            cells.push(TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(text))));
        }
        table = table.add_row(TableRow::new(cells));
    }
    // 将表格添加到文档中
    doc = doc.add_table(table);

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


 