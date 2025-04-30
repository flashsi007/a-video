use crate::db::modes::VideoInfo;
use once_cell::sync::Lazy;
use rusqlite::{Connection, Result, Row}; 
use std::path::Path;
use std::sync::{Arc, Mutex}; use crate::app_handle::get_app_handle; 
use tauri::Manager;
use crate::utils::utils::get_current_date;
pub struct SqliteDB {
    pub conn: Arc<Mutex<Connection>>,
}

impl SqliteDB {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(SqliteDB {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute(sql, params)
    }

    pub fn query<T, F>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
        mut map_fn: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&Row) -> Result<T>,
    {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, |row| map_fn(row))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn create_table(&self, sql: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(sql, [])?;
        Ok(())
    }

    // 插入视频信息到数据库
    pub async fn insert_video_info(&self, video_info: &VideoInfo) -> Result<()> {
        let sql = "INSERT INTO video_info (vod_id,vod_type_id, title, img_url, type_name, year, area, language, description, director, actor, video_urls,lzzy_video_urls,created_at,updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12 , ?13, ?14, ?15)";

        self.execute(
            sql,
            &[
                &video_info.vod_id as &dyn rusqlite::ToSql,
                &video_info.vod_type_id,
                &video_info.title,
                &video_info.img_url,
                &video_info.type_name,
                &video_info.year,
                &video_info.area,
                &video_info.language,
                &video_info.description,
                &video_info.director,
                &video_info.actor,
                &video_info.video_urls,
                &video_info.lzzy_video_urls,
                &video_info.created_at,
                &video_info.updated_at,
            ],
        )?;

        Ok(())
    }

    // 初始化数据库表
    pub fn init_tables(&self) -> Result<()> {
        let create_video_info_table = "
            CREATE TABLE IF NOT EXISTS video_info (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                vod_id TEXT NOT NULL, 
                vod_type_id TEXT NOT NULL,
                title TEXT NOT NULL,
                img_url TEXT NOT NULL,
                type_name TEXT NOT NULL,
                year TEXT NOT NULL,
                area TEXT NOT NULL,
                language TEXT NOT NULL,
                description TEXT NOT NULL,
                director TEXT NOT NULL,
                actor TEXT NOT NULL,
                video_urls TEXT NOT NULL,
                lzzy_video_urls TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )";

        self.create_table(create_video_info_table)
    }

    // 检查视频是否已存在于数据库中
    pub async fn check_video_exists(&self, vod_id: &str) -> Result<bool> {
        let sql = "SELECT COUNT(*) FROM video_info WHERE vod_id = ?1";

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let count: i64 = stmt.query_row(&[vod_id], |row| row.get(0))?;

        Ok(count > 0)
    }

    // 更新视频信息中的img_url和video_urls字段
    pub async fn update_video_info(
        &self,
        vod_id: &str,
        img_url: &str,
        video_urls: &str,
    ) -> Result<()> {
        //   返回当前日期时间的字符串
        let updated_at = get_current_date(); 
       let sql = "UPDATE video_info SET img_url = ?1, video_urls = ?2, updated_at = ?3 WHERE vod_id = ?4";

    self.execute(
        sql,
        &[
            &img_url as &dyn rusqlite::ToSql, 
            &video_urls as &dyn rusqlite::ToSql, 
            &updated_at as &dyn rusqlite::ToSql, 
            &vod_id as &dyn rusqlite::ToSql
        ],
    )?;

    Ok(())
    }

    pub async fn update_video_lzzy_video_urls(
        &self,
        vod_id: &str,
        lzzy_video_urls: &str,
    ) -> Result<()> {
        let sql = "UPDATE video_info SET lzzy_video_urls =?1 WHERE vod_id =?2";

        self.execute(sql, &[&lzzy_video_urls as &dyn rusqlite::ToSql, &vod_id])?;

        Ok(())
    }

    // 根据 vod_name 检查视频是否已存在于数据库中
    pub async fn check_video_exists_by_vod_name(&self, vod_name: &str) -> Result<bool> {
        let sql = "SELECT COUNT(*) FROM video_info WHERE title =?1";

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let count: i64 = stmt.query_row(&[vod_name], |row| row.get(0))?;

        Ok(count > 0)
    }

    // 可根据需要扩展更多通用方法，如批量插入、事务等
}


fn get_db_path() -> Result<String, rusqlite::Error> {
    let app_handle = get_app_handle().lock().map_err(|_| rusqlite::Error::InvalidPath("AppHandle lock failed".to_string().into()))?;
    let mut path = app_handle.path().resource_dir().map_err(|_| rusqlite::Error::InvalidPath("Resource dir not found".to_string().into()))?;
    path.push("video.db");
    Ok(path.to_str().unwrap().to_string())
}
 
pub static DB_INSTANCE: Lazy<SqliteDB> = Lazy::new(|| {
    
    let path = get_db_path();

    println!("数据库路径: {:?}", path); 
    let db = SqliteDB::new(path.unwrap())
        .expect("数据库初始化失败");
    // 初始化表结构
    db.init_tables().expect("表初始化失败");
    db
});
