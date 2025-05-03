mod db; 
mod utils;  
mod collect;
mod structs;
mod package;
mod commands;  
mod services;
mod app_handle; 

use crate::services::vod_service; 
use crate::commands::{set_db_path,get_vod_types,crawl_ffzy}; 
 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![ 
            crawl_ffzy,
            vod_service::query_videos,
            set_db_path,
            get_vod_types
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
