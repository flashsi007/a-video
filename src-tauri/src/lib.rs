
mod crawler;
mod db;
mod services;
use crate::crawler::ffzy; 
use crate::services::vod_service; 
use std::fs;  
mod utils;
use crate::utils::resource_utils::get_resource_path;
 
mod app_handle;
use crate::app_handle::get_app_handle; 

mod structs;
use crate::structs::structs::CrawlParams;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn crawl_ffzy(window: tauri::Window,params: CrawlParams) -> Result<String, String> {
    ffzy::run(window,params).await
}
 
#[tauri::command]
fn get_vod_types() -> Result<String, String> {  
    let app_handle = get_app_handle().lock().unwrap();
    let  resource_path = get_resource_path(&app_handle, "resources/crawler/vod_type.json").unwrap();
    fs::read_to_string(&resource_path).map_err(|e| format!("读取vod_type.json失败: {}", e))
 
}


 

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet,  
                crawl_ffzy,  
                vod_service::query_videos,
                get_vod_types, 
            ])
            .setup(|app| {
                // 初始化全局 AppHandle 
               let handle = app.handle().clone();
                app_handle::set_app_handle(handle); 
                Ok(())
            })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
