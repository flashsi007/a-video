
use crate::package::fetch::fetch;
use crate::db::modes::VideoInfo;
use crate::db::sqlite::DB_INSTANCE as db;
use crate::structs::structs::CollectType;
use crate::structs::structs::{ResponseLzzyVod,Progress};
use serde::Deserialize;
use serde_json::Value;  
use std::sync::Arc;
use tauri::Emitter;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::utils::utils::get_current_date;
use crate::utils::resource_utils::get_resource_path;
use crate::app_handle::get_app_handle;
use std::fs;


fn calculate_total_pages(total: i32, limit: i32) -> i32 {
    if limit == 0 {
        panic!("Limit cannot be zero");
    }
    let mut total_pages = total / limit;
    if total % limit != 0 {
        total_pages += 1;
    }
    total_pages
}

async fn lzzy_insert_video_info(video_info: VideoInfo) {
    match db.check_video_exists_by_vod_name(&video_info.title).await {
        Ok(exists) => {
            if exists {
                if let Err(e) = db.update_video_lzzy_video_urls(  &video_info.img_url, &video_info.vod_id,  &video_info.lzzy_video_urls).await {
                    println!("更新视频信息失败: {}", e);
                    // 弹窗提示
                    if let Some(window) = get_app_handle().lock().unwrap().get_webview_window("main") {
                        let _ = window.emit("show_error_dialog", format!("更新视频信息失败: {}", e));
                    }
                } else {
                    println!("更新视频信息成功: {}", video_info.vod_id);
                }
            } else {
                if let Err(e) = db.insert_video_info(&video_info).await {
                    println!("插入数据库失败: {}", e);
                    // 弹窗提示
                    if let Some(window) = get_app_handle().lock().unwrap().get_webview_window("main") {
                        let _ = window.emit("show_error_dialog", format!("插入数据库失败: {}", e));
                    }
                } else {
                    println!("插入视频信息成功: {}", video_info.vod_id);
                }
            }
        }
        Err(e) => {
            println!("检查视频是否存在失败: {}", e);
            if let Some(window) = get_app_handle().lock().unwrap().get_webview_window("main") {
                let _ = window.emit("show_error_dialog", format!("检查视频是否存在失败: {}", e));
            }
            if let Err(e) = db.insert_video_info(&video_info).await {
                println!("插入数据库失败: {}", e);
                if let Some(window) = get_app_handle().lock().unwrap().get_webview_window("main") {
                    let _ = window.emit("show_error_dialog", format!("插入数据库失败: {}", e));
                }
            }
        }
    }
}

// 量子资源  
pub async fn get_lzzy_vod_detail(collect_type: CollectType) {
    // 获取AppHandle以便发送进度信息
    let app_handle = crate::app_handle::get_app_handle().lock().unwrap().clone();

    // 创建进度对象，使用Arc和Mutex包装以便在线程间共享
    let progress = Arc::new(Mutex::new(Progress {
        percent: 0.0,
        current: 0,
        total: 0,
        message: "准备采集...".to_string(),
    }));

    let base_url = match collect_type {
        CollectType::当天采集 => {
            "https://cj.lziapi.com/api.php/provide/vod?ac=detail&ct=1&h=24"
        }
        CollectType::一周采集 => {
            "https://cj.lziapi.com/api.php/provide/vod?ac=detail&ct=1&h=168"
        }
        CollectType::所有采集 => "https://cj.lziapi.com/api.php/provide/vod?ac=detail&ct=1",
    };

    // 在子线程中执行采集任务
    tokio::spawn(async move {
        let path = format!("{}{}", base_url, "&pg=1");
        // 假设 fetch 返回 Result<Value, Error>
        let json_data: Value = match fetch(&path, "json", None).await {
            Ok(result) => result,
            Err(e) => {
                println!("获取数据失败: {}", e);
                return;
            }
        };

        // 使用 from_value 替代 from_str
        let response: ResponseLzzyVod = match serde_json::from_value(json_data) {
            Ok(data) => data,
            Err(e) => {
                println!("解析JSON失败: {}", e);
                return;
            }
        };

        let total = response.total;
        let limit = response.limit.parse::<i32>().unwrap();
        let total_pages = calculate_total_pages(total, limit);

        let mut urls = Vec::new();
        for i in 1..=total_pages {
            let path = format!("{}&pg={}", base_url, i);
            urls.push(path);
        }

        // 更新总数量
        {
            let mut guard = progress.lock().await;
            guard.total = total;
            guard.message = "开始采集...".to_string();
            // 发送进度信息给前端
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("ffzy_progress", guard.clone());
            }
        }

        let mut processed_count = 0;

        // 控制进度事件发送频率
        let emit_interval = 50; // 每处理50个视频发送一次进度

        for (page_index, url) in urls.iter().enumerate() {
            let json_data: Value = match fetch(url, "json", None).await {
                Ok(result) => result,
                Err(e) => {
                    println!("获取数据失败: {}", e);
                    continue;
                }
            };

            let response: ResponseLzzyVod = match serde_json::from_value(json_data) {
                Ok(data) => data,
                Err(e) => {
                    println!("解析JSON失败: {}", e);
                    continue;
                }
            };
            let date =  get_current_date();
            // 顺序插入，避免高并发
            for vod in response.list {
                let video_info = VideoInfo {
                    id: None,
                    vod_id: vod.vod_id.to_string(),
                    vod_type_id: vod.type_id.to_string(),
                    title: vod.vod_name.clone(),
                    img_url: vod.vod_pic.clone(),
                    type_name: vod.type_name.clone(),
                    year: vod.vod_year.clone(),
                    area: vod.vod_area.clone(),
                    language: vod.vod_lang.clone(),
                    description: vod.vod_blurb.clone(),
                    director: vod.vod_director.clone(),
                    actor: vod.vod_actor.clone(),
                    video_urls: "".to_string(),
                    lzzy_video_urls: vod.vod_play_url.clone(),
                    created_at: date.clone(),
                    updated_at: date.clone(),
                };

                // 顺序执行数据库操作，避免spawn导致锁冲突
                lzzy_insert_video_info(video_info).await;

                // 更新进度信息
                processed_count += 1;
                if processed_count % emit_interval == 0 || (page_index == urls.len() - 1 && processed_count == total) {
                    let mut guard = progress.lock().await;
                    guard.current = processed_count;
                    guard.percent = (processed_count as f32 / total as f32) * 100.0;
                    guard.message = "正在采集...".to_string();
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.emit("ffzy_progress", guard.clone());
                    }
                }
            }
        }

        // 采集完成，发送最终进度
        {
            let mut guard = progress.lock().await;
            guard.percent = 100.0;
            guard.message = "采集完成".to_string();
            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.emit("ffzy_progress", guard.clone());
            }
        }
    });
}


// 设置数据库地址 
#[tauri::command]
pub fn  set_db_path (db_path:&str)-> String {
    println!("Database path set to: {}", db_path);
    format!("{}", db_path)
}

#[tauri::command]
pub fn get_vod_types() -> Result<String, String> {
    let app_handle = get_app_handle().lock().unwrap();
    let resource_path = get_resource_path(&app_handle, "resources/crawler/vod_type.json").unwrap();
    fs::read_to_string(&resource_path).map_err(|e| format!("读取vod_type.json失败: {}", e))
}
