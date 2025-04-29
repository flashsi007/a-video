// 数据模型定义示例
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    pub id: Option<i32>,
    pub vod_id: String,
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