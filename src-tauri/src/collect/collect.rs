use crate::db::modes::{VideoInfo, CollectRecord};
use crate::db::sqlite::DB_INSTANCE as db;
use crate::structs::structs::CollectType;
use crate::structs::structs::{ResponseLzzyVod,Progress};
use crate::utils::log;
use serde_json::Value;  
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use tauri::Emitter;
use tauri::Manager;
use tokio::sync::Mutex;
use crate::utils::utils::get_current_date; 
use crate::app_handle::get_app_handle; 
use crate::package::fetch::fetch;

const   CHUNK: usize = 100; // 分成 100 个 chunk

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
                if let Err(e) = db.update_video_lzzy_video_urls(&video_info.img_url, &video_info.vod_id, &video_info.lzzy_video_urls).await {
                    println!("更新视频信息失败: {}", e);
                    if let Some(window) = get_app_handle().lock().unwrap().get_webview_window("main") {
                        let _ = window.emit("show_error_dialog", format!("更新视频信息失败: {}", e));
                    }
                } else {
                    println!("更新视频信息成功: {}", video_info.vod_id); 
                    let message = format!("更新视频信息成功: {}", video_info.vod_id);
                    log::info(&message);
                }
            } else {
                if let Err(e) = db.insert_video_info(&video_info).await {
                    println!("插入数据库失败: {}", e);
                    if let Some(window) = get_app_handle().lock().unwrap().get_webview_window("main") {
                        let _ = window.emit("show_error_dialog", format!("插入数据库失败: {}", e));
                        let message = format!("插入数据库失败: {}", e);
                        log::error(&message);
                    }
                } else {
                    println!("插入视频信息成功: {}", video_info.vod_id);
                    let message = format!("更新视频信息成功: {}", video_info.vod_id);
                    log::info(&message);
                }
            }
        }
        Err(e) => {
            println!("检查视频是否存在失败: {}", e);
            let message = format!("检查视频是否存在失败: {}", e);
            log::error(&message);

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

pub async fn get_lzzy_vod_detail(collect_type: CollectType) {
  

    let collect_type_num = match collect_type {
        CollectType::当天采集 => 0,
        CollectType::一周采集 => 1,
        CollectType::所有采集 => 2,
    };
    
    let mut start_page = 1;
    if let Ok(Some(record)) = db.get_uncompleted_collect_record(collect_type_num).await {
        start_page = record.current_page;
        println!("发现未完成采集任务，从第{}页继续", start_page);
    } else {
        let new_record = CollectRecord {
            id: None,
            collect_type: collect_type_num,
            current_page: 1,
            total_pages: 0,
            processed_count: 0,
            total_count: 0,
            is_completed: false,
            created_at: crate::utils::utils::get_current_date(),
            updated_at: crate::utils::utils::get_current_date(),
        };
        if let Err(e) = db.create_collect_record(&new_record).await {
            println!("创建采集记录失败: {}", e);
        }
    }

    let app_handle = crate::app_handle::get_app_handle().lock().unwrap().clone();
    let progress = Arc::new(Mutex::new(Progress {
        percent: 0.0,
        current: 0,
        total: 0,
        message: "准备采集...".to_string(),
    }));

    let base_url = match collect_type {
        CollectType::当天采集 => "https://cj.lziapi.com/api.php/provide/vod?ac=detail&ct=1&h=24",
        CollectType::一周采集 => "https://cj.lziapi.com/api.php/provide/vod?ac=detail&ct=1&h=168",
        CollectType::所有采集 => "https://cj.lziapi.com/api.php/provide/vod?ac=detail&ct=1",
    };

    // 分割URL列表为5个部分并行处理
    let path = format!("{}{}", base_url, "&pg=1");
    let json_data: Value = match fetch(&path, "json", None).await {
        Ok(result) => result,
        Err(e) => {
            println!("获取数据失败: {}", e);
            return;
        }
    };

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
    for i in start_page..=total_pages {
        urls.push(format!("{}&pg={}", base_url, i));
    }

    {
        let mut guard = progress.lock().await;
        guard.total = total;
        guard.message = "开始采集...".to_string();
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.emit("ffzy_progress", guard.clone());
        }
    }

    let emit_interval = 50i32;
    let urls_len =  urls.len();
    let chunk_size = (urls_len+ 4) / CHUNK; 
    let mut handles = vec![];
    let processed_count = Arc::new(AtomicI32::new(0));

    for chunk in urls.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let app_handle = app_handle.clone();
        let progress = progress.clone();
        let start_page = start_page;
        let collect_type_num = collect_type_num;
        let emit_interval = emit_interval;
        let total = total;
        let limit = limit;

        let processed_count = processed_count.clone();
        let handle = tokio::spawn(async move {
            let chunk_start_page = if let Some(first_url) = chunk.first() {
                first_url.split("pg=").last().unwrap().parse::<i32>().unwrap()
            } else {
                return;
            };

            for (page_index, url) in chunk.iter().enumerate() {
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

                if let Err(e) = db.update_collect_page(collect_type_num, (page_index as i32) + start_page).await {
                    println!("更新采集页码失败: {}", e);
                }

                let date = get_current_date();
                let list = response.list;
                let list_len = list.len() as i32;
                for vod in list {
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

                    lzzy_insert_video_info(video_info).await;

                    let current_count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
                    if current_count % emit_interval == 0 || (page_index == chunk.len() - 1 && current_count == list_len * (page_index as i32 + 1)) {
                        let mut guard = progress.lock().await;
                        // 计算总处理量(当前处理量 + 已跳过页面的处理量)
                        let total_processed = current_count + ((start_page - 1) * limit);
                        guard.current = total_processed + emit_interval;
                        guard.percent = (total_processed as f32 / total as f32) * 100.0;
                        guard.message = "正在采集...".to_string();
                        
                        if let Err(e) = db.update_collect_progress(
                            collect_type_num,
                            total_processed,
                            total,
                            (page_index as i32) + start_page
                        ).await {
                            println!("更新采集进度失败: {}", e);
                        }

                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.emit("ffzy_progress", guard.clone());
                        }
                    }
                }
            }

            // 当前chunk处理完成
            {
                let mut guard = progress.lock().await;
                guard.message = format!("部分采集完成: {}-{}页", 
                    chunk_start_page, 
                    chunk_start_page + chunk.len() as i32 - 1);
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.emit("ffzy_progress", guard.clone());
                }
            }
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        let _ = handle.await;
    }

    {
        if let Err(e) = db.complete_collect_record(collect_type_num).await {
            println!("标记采集完成失败: {}", e);
        }
        let mut guard = progress.lock().await;
        guard.percent = 100.0;
        guard.message = "采集完成".to_string();
        let message = format!("采集完成一共: {} 条", total);
        log::info(&message);
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.emit("ffzy_progress", guard.clone());
        }
    }
}
