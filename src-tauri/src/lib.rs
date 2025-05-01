mod package;
mod db; 


mod commands;
use tauri::Manager;

use crate::commands::{get_lzzy_vod_detail,set_db_path,get_vod_types};

mod services;
use crate::services::vod_service;

mod utils;


mod app_handle;


mod structs;
use crate::structs::structs::CollectType;
 

#[tauri::command]
async fn crawl_ffzy(status: i32) -> Result<String, String> {
    let collect_type = match status {
        0 => CollectType::当天采集,
        1 => CollectType::一周采集,
        2 => CollectType::所有采集,
        _ => panic!("无效的值"),
    };

    // 在子线程中执行采集任务，并向前端发送进度信息
    get_lzzy_vod_detail(collect_type).await;

    Ok("success".to_string())
}






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
            // 生产环境下强制启用开发者工具
            let window = app.get_webview_window("main").unwrap(); 
            #[warn(debug_assertions)]
            { 
                let _ = window.open_devtools();
                let _ = window.set_enabled(true); // 允许操作
            }

              
            // 初始化全局 AppHandle
            let handle = app.handle().clone();
            app_handle::set_app_handle(handle);
            
            Ok(()) 
        })
        
          
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
