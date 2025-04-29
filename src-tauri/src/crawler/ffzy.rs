 
use tauri::Emitter;
use crate::db::modes::VideoInfo;
use crate::crawler::package::config::API_BASE_URL;
use crate::crawler::package::fetch;
use crate::crawler::package::html_parser;
use crate::crawler::package::url_utils;
use regex::Regex;
use tauri::Window;

use crate::db::sqlite::DB_INSTANCE as db;

/// 提取 URL 中的数字部分
fn extract_id_from_url(url: &str) -> Option<String> {
    // 定义正则表达式，匹配路径中的数字部分
    let re = Regex::new(r"/(\d+)\.html$").unwrap();
    
    // 使用正则表达式匹配 URL
    if let Some(captures) = re.captures(url) {
        // 提取第一个捕获组（即数字部分）
        if let Some(matched) = captures.get(1) {
            // 返回匹配到的字符串作为 String 类型
            return Some(matched.as_str().to_string());
        }
    }
    
    // 如果没有匹配到，返回 None
    None
}


/// 提取 URL 中的数字部分
fn vod_type_id_from_url(url: &str) -> Option<String> {
    // 匹配类似 /id/123 这样的结构，并捕获数字部分
    let re = Regex::new(r"/id/(\d+)").unwrap();

    if let Some(captures) = re.captures(url) {
        if let Some(matched) = captures.get(1) {
            return Some(matched.as_str().to_string());
        }
    }

    None
}
 
 

fn extract_field(html_str: &str, selector: &str) -> Option<String> {
    html_parser::extract_disabled_text(html_str, selector)
}

fn clean_field(field: Option<String>, prefix: &str) -> Option<String> {
    field.map(|f| f.replace(prefix, "").trim().to_string())
}

fn extract_img(html_str: &str, selector: &str) -> Option<String> {
    html_parser::extract_img_src(html_str, selector)
}

fn extract_links(html_str: &str, selector: &str) -> Option<Vec<String>> {
    html_parser::extract_all_links(html_str, selector)
}

// fn print_video_info(vod_id: &str, title: &str, img_url: &str, type_name: &str, year: &str, area: &str, language: &str, description: &str, director: &str, actor: &str, video_url: &Vec<String>) {
//     println!("{}", vod_id);
//     println!("{}", title);
//     println!("封面图 {}", img_url);
//     println!("{}", type_name);
//     println!("{}", year);
//     println!("{}", area);
//     println!("{}", language);
//     println!("{}", description);
//     println!("{}", director);
//     println!("{}", actor);
//     println!("{:?} \n", video_url);
// }


 
pub async fn run(window: Window) -> Result<String, String> {
    let progress = std::sync::Arc::new(tokio::sync::Mutex::new((0usize, 0)));

    let urls = url_utils::build_type_urls("src/crawler/vod_type.json").map_err(|e| e.to_string())?;
   
      
    // 用于存储所有子任务的handle
    let mut handles = vec![];
    

    for url in urls {

        let vod_type_id = match vod_type_id_from_url(&url) {
            Some(id) => id,
            None => {
                println!("未能提取到 vod_type_id。");
                return Ok("".to_string());
            }
        };

        let html = fetch::fetch(&url, "html", None).await.map_err(|e| e.to_string())?;
    
        let html_str = match html.as_str() {
                Some(s) => s,
                None => {
                    println!("fetch结果不是字符串格式");
                    return Ok("".to_string());
                }
        };
        let selector_str = "body > div.container > div.kscont > div > span.disabled";
        let disabled_text = match html_parser::extract_disabled_text(html_str, selector_str) {
                Some(t) => t,
                None => {
                    println!("未找到目标元素。");
                    return Ok("".to_string());
                }
        };

        let total_pages = match html_parser::extract_total_pages_from_text(&disabled_text) {
                Some(p) => p,
                None => {
                    println!("未能提取到总页数。");
                    return Ok("".to_string());
                }
        };

        

        let url_clone = url.clone();
        let window_clone = window.clone();
        let progress_clone = progress.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = process_vod_type_with_progress(&url_clone, &vod_type_id,window_clone, progress_clone, total_pages  ).await {
                println!("处理URL失败: {}，错误: {}", url_clone, e);
            }
        });
        handles.push(handle);
    }



    for handle in handles {
        let _ = handle.await;
    }
    Ok("".to_string())
}

// 新增带进度的处理函数
async fn process_vod_type_with_progress(url: &str, vod_type_id: &str,window: Window, progress: std::sync::Arc<tokio::sync::Mutex<(usize, usize)>>, progress_total: usize) -> Result<(), Box<dyn std::error::Error>> {
  
    let html = fetch::fetch(&url, "html", None).await.map_err(|e| e.to_string())?;
    let html_str = match html.as_str() {
        Some(s) => s,
        None => {
            println!("fetch结果不是字符串格式");
            return Ok(());
        }
    };

    let selector_str = "body > div.container > div.kscont > div > span.disabled";
    let disabled_text = match html_parser::extract_disabled_text(html_str, selector_str) {
        Some(t) => t,
        None => {
            println!("未找到目标元素。");
            return Ok(());
        }
    };

    let total_pages = match html_parser::extract_total_pages_from_text(&disabled_text) {
        Some(p) => p,
        None => {
            println!("未能提取到总页数。");
            return Ok(());
        }
    };

    process_paged_urls_with_progress(&url,vod_type_id, total_pages.try_into().unwrap(), window, progress, progress_total).await
}

async fn process_paged_urls_with_progress(base_url: &str, vod_type_id: &str,total_pages: u32, window: Window, progress: std::sync::Arc<tokio::sync::Mutex<(usize, usize)>>, progress_total: usize) -> Result<(), Box<dyn std::error::Error>> 
{
    let paged_urls = url_utils::generate_paged_urls(base_url, total_pages);

    // 外层循环：依次处理每个分页 URL
    for paged_url in paged_urls.iter() { 
    
        // 获取当前页面 HTML 内容
        let html = fetch::fetch(paged_url, "html", None).await.map_err(|e| e.to_string())?;
        let html_str = match html.as_str() {
            Some(s) => s,
            None => continue,
        };

        // 提取当前页面上的所有视频链接
        let selector_str = "body > div.container > div.addition-content > ul > li > a.videoName";
        let url_list =  html_parser::extract_all_links(html_str, selector_str) ;

        // 内层循环：依次处理每个视频链接
        for urls in url_list.iter() { 
            for url  in  urls{
                let base_url = format!("{}{}",API_BASE_URL,url);
                println!(" 开始处理URL: {}", base_url);
                // 调用处理函数，并等待其完成 
               process_video_page_with_progress(&base_url,vod_type_id, window.clone(), progress.clone(), progress_total*urls.len()).await?;
            }
 
        }
    }

    Ok(())
}

async fn process_video_page_with_progress( url: &str,vod_type_id: &str,  window: Window,  progress: std::sync::Arc<tokio::sync::Mutex<(usize, usize)>>, progress_total: usize) -> Result<(), Box<dyn std::error::Error>> {

   // 进度统计与事件推送
   let mut guard = progress.lock().await;
    

    let html = fetch::fetch(url, "html", None).await.map_err(|e| e.to_string())?;
    let html_str = match html.as_str() {
        Some(s) => s,
        None => return Ok(()),
    };

    let vod_id = extract_id_from_url(&url).unwrap_or_default();
    let img_url = extract_img(html_str, "body > div:nth-child(18) > div > div.left > img").unwrap_or_default();
    let title = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(1)"), "片名：").unwrap_or_default();
    let type_name = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(5)"), "类型：").unwrap_or_default();
    let director = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(6)"), "导演：").unwrap_or_default();
    let actor = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(7)"), "演员：").unwrap_or_default();
    let year = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(8)"), "年代：").unwrap_or_default();
    let area = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(9)"), "地区：").unwrap_or_default();
    let language = clean_field(extract_field(html_str, "body > div:nth-child(18) > div > div.right > p:nth-child(10)"), "语言：").unwrap_or_default();
    let description = extract_field(html_str, "#content > div > p").unwrap_or_default();
    let video_url = extract_links(html_str, "#content > div > div > div > li > a:nth-child(2)").unwrap_or(vec![]);
    let video_urls_json = serde_json::to_string(&video_url).unwrap_or("[]".to_string());
    
    let video_info = VideoInfo {
        id: None,
        vod_id: vod_id.clone(),
        vod_type_id:vod_type_id.to_string(),
        title: title.clone(),
        img_url: img_url.clone(),
        type_name: type_name.clone(),
        year: year.clone(),
        area: area.clone(),
        language: language.clone(),
        description: description.clone(),
        director: director.clone(),
        actor: actor.clone(),
        video_urls: video_urls_json.clone(),
    };

    let video_info_clone = video_info.clone();
    
    tokio::spawn(async move {
        match db.check_video_exists(&video_info_clone.vod_id).await {
            Ok(exists) => {
                if exists {
                    if let Err(e) = db.update_video_info(&video_info_clone.vod_id, &video_info_clone.img_url, &video_info_clone.video_urls).await {
                        println!("更新视频信息失败: {}", e);
                    } else {
                        println!("更新视频信息成功: {}", video_info_clone.vod_id);
                    }
                } else {
                    if let Err(e) = db.insert_video_info(&video_info_clone).await {
                        println!("插入数据库失败: {}", e);
                    } else {
                        println!("插入视频信息成功: {}", video_info_clone.vod_id);
                    }
                }
            },
            Err(e) => {
                println!("检查视频是否存在失败: {}", e);
                if let Err(e) = db.insert_video_info(&video_info_clone).await {
                    println!("插入数据库失败: {}", e);
                }
            }
        }
    }); 
    guard.0 += 1;
    let progress_str = format!("{}/{}", guard.0, progress_total);
    let _ = window.emit("ffzy_progress", progress_str);
 
      Ok(())
}
 
 