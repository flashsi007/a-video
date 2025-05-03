use serde::{Deserialize, Serialize};

#[repr(u8)]
pub enum CollectType {
    当天采集 = 0, // 当天采集
    一周采集 = 1, // 一周采集
    所有采集 = 2, // 所有采集
}

/// 日志类型枚举
pub enum LogType {
    Info,
    Error,
    Success,
    Warning,
}


/// 日志记录器结构体
pub struct Logger {
    pub log_file_path: String,
}

#[derive(Debug)]
pub enum ResourceError {
    DirNotFound,
    FileNotFound(String),
    IoError(String),
    JsonError(String),
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub percent: f32,
    pub current: i32,
    pub total: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct CrawlParams {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LzzyVod {
    pub vod_id: i32,
    pub type_id: i32,
    pub type_id_1: i32,
    pub group_id: i32,
    pub vod_name: String,
    pub vod_sub: String,
    pub vod_en: String,
    pub vod_status: i32,
    pub vod_letter: String,
    pub vod_color: String,
    pub vod_tag: String,
    pub vod_class: String,
    pub vod_pic: String,
    pub vod_pic_thumb: String,
    pub vod_pic_slide: String,
    pub vod_pic_screenshot: String,
    pub vod_actor: String,
    pub vod_director: String,
    pub vod_writer: String,
    pub vod_behind: String,
    pub vod_blurb: String,
    pub vod_remarks: String,
    pub vod_pubdate: String,
    pub vod_total: i32,
    pub vod_serial: String,
    pub vod_tv: String,
    pub vod_weekday: String,
    pub vod_area: String,
    pub vod_lang: String,
    pub vod_year: String,
    pub vod_version: String,
    pub vod_state: String,
    pub vod_author: String,
    pub vod_jumpurl: String,
    pub vod_tpl: String,
    pub vod_tpl_play: String,
    pub vod_tpl_down: String,
    pub vod_isend: i32,
    pub vod_lock: i32,
    pub vod_level: i32,
    pub vod_copyright: i32,
    pub vod_points: i32,
    pub vod_points_play: i32,
    pub vod_points_down: i32,
    pub vod_hits: i32,
    pub vod_hits_day: i32,
    pub vod_hits_week: i32,
    pub vod_hits_month: i32,
    pub vod_duration: String,
    pub vod_up: i32,
    pub vod_down: i32,
    pub vod_score: String,
    pub vod_score_all: i32,
    pub vod_score_num: i32,
    pub vod_time: String,
    pub vod_time_add: i64,
    pub vod_time_hits: i32,
    pub vod_time_make: i32,
    pub vod_trysee: i32,
    pub vod_douban_id: i32,
    pub vod_douban_score: String,
    pub vod_reurl: String,
    pub vod_rel_vod: String,
    pub vod_rel_art: String,
    pub vod_pwd: String,
    pub vod_pwd_url: String,
    pub vod_pwd_play: String,
    pub vod_pwd_play_url: String,
    pub vod_pwd_down: String,
    pub vod_pwd_down_url: String,
    pub vod_content: String,
    pub vod_play_from: String,
    pub vod_play_server: String,
    pub vod_play_note: String,
    pub vod_play_url: String,
    pub vod_down_from: String,
    pub vod_down_server: String,
    pub vod_down_note: String,
    pub vod_down_url: String,
    pub vod_plot: i32,
    pub vod_plot_name: String,
    pub vod_plot_detail: String,
    pub type_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseLzzyVod {
    pub code: i32,
    pub msg: String,
    pub page: String,
    pub pagecount: i32,
    pub limit: String,
    pub total: i32,
    pub list: Vec<LzzyVod>,
}
