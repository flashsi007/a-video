
mod crawler;
mod db;
use crate::crawler::ffzy;
use db::db_config::DbConfig;
use tauri::Manager;
use tauri::Emitter;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn crawl_ffzy(window: tauri::Window) -> Result<String, String> {
    ffzy::run(window).await
}

#[tauri::command]
 async fn check_and_set_db_path(window: tauri::WebviewWindow) -> Result<String, String> {
    let config = DbConfig::load();
    if config.db_path.is_none() {
        window.emit("select-db-path", ()).map_err(|e| format!("事件发送失败: {}", e))?;
        Err("请在前端选择数据库路径后重新尝试。".to_string())
    } else {
        Ok(config.db_path.unwrap())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, 
            crawl_ffzy, 
            check_and_set_db_path
            ])
        .setup(|app_handle| {
            let window = app_handle.get_webview_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                let _ =  check_and_set_db_path(window).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
