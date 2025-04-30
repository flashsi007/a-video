mod crawler;
mod db; 
use std::fs;

mod commands;
use crate::commands::{get_lzzy_vod_detail,set_db_path};

mod services;
use crate::services::vod_service;

mod utils;
use crate::utils::resource_utils::get_resource_path;

mod app_handle;
use crate::app_handle::get_app_handle;

mod structs;
use crate::structs::structs::CollectType;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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

#[tauri::command]
fn get_vod_types() -> Result<String, String> {
    let app_handle = get_app_handle().lock().unwrap();
    let resource_path = get_resource_path(&app_handle, "resources/crawler/vod_type.json").unwrap();
    fs::read_to_string(&resource_path).map_err(|e| format!("读取vod_type.json失败: {}", e))
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
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
