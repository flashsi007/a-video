// 数据模型定义示例
use serde::{Serialize, Deserialize};

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
    pub video_urls: String // 可以用JSON字符串存储Vec<String>
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
        })
    }
}