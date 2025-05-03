// 数据模型定义示例
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    pub id: Option<i32>,
    pub vod_id: String,
    pub vod_type_id: String,
    pub title: String,
    pub img_url: String,
    pub type_name: String,
    pub year: String,
    pub area: String,
    pub language: String,
    pub description: String,
    pub director: String,
    pub actor: String,
    pub video_urls: String,      // 可以用JSON字符串存储Vec<String>
    pub lzzy_video_urls: String, // 量子资源 
    pub created_at: String,
    pub updated_at: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectRecord {
    pub id: Option<i32>,
    pub collect_type: i32,       // 0: 当天采集, 1: 一周采集, 2: 所有采集
    pub current_page: i32,       // 当前采集页码
    pub total_pages: i32,        // 总页数
    pub processed_count: i32,    // 已处理数量
    pub total_count: i32,        // 总数量
    pub is_completed: bool,      // 是否完成
    pub created_at: String,      // 创建时间
    pub updated_at: String       // 更新时间
}

impl VideoInfo {
    pub fn from_row(row: &rusqlite::Row) -> Result<VideoInfo, rusqlite::Error> {
        Ok(VideoInfo {
            id: row.get("id").ok(),
            vod_id: row.get("vod_id")?,
            vod_type_id: row.get("vod_type_id")?,
            title: row.get("title")?,
            img_url: row.get("img_url")?,
            type_name: row.get("type_name")?,
            year: row.get("year")?,
            area: row.get("area")?,
            language: row.get("language")?,
            description: row.get("description")?,
            director: row.get("director")?,
            actor: row.get("actor")?,
            video_urls: row.get("video_urls")?,
            lzzy_video_urls: row.get("lzzy_video_urls")?, // 量子资源
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}
