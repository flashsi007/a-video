
use crate::collect::collect::get_lzzy_vod_detail;  
use crate::structs::structs::CollectType; 
use crate::utils::resource_utils::get_resource_path; 
use crate::app_handle::get_app_handle; 
use std::fs;




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


#[tauri::command]
pub async fn crawl_ffzy(status: i32) -> Result<String, String> {
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



