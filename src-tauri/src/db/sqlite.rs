use crate::db::modes::{VideoInfo, CollectRecord};
use once_cell::sync::Lazy;
use rusqlite::{Connection, Result, Row}; 
use std::path::Path;
use std::sync::{Arc, Mutex}; 
use crate::app_handle::get_app_handle; 
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

        let create_collect_records_table = "
            CREATE TABLE IF NOT EXISTS collect_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                collect_type INTEGER NOT NULL,
                current_page INTEGER NOT NULL,
                total_pages INTEGER NOT NULL,
                processed_count INTEGER NOT NULL,
                total_count INTEGER NOT NULL,
                is_completed BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )";

        self.create_table(create_video_info_table)?;
        self.create_table(create_collect_records_table)
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

    pub async fn update_video_lzzy_video_urls( &self, img_url: &str,  vod_id: &str,   lzzy_video_urls: &str,  ) -> Result<()> {
        //   返回当前日期时间的字符串
        let updated_at = get_current_date(); 
       let sql = "UPDATE video_info SET img_url = ?1, lzzy_video_urls = ?2, updated_at = ?3 WHERE vod_id = ?4";

    self.execute(
        sql,
        &[
            &img_url as &dyn rusqlite::ToSql, 
            &lzzy_video_urls as &dyn rusqlite::ToSql, 
            &updated_at as &dyn rusqlite::ToSql, 
            &vod_id as &dyn rusqlite::ToSql
        ],
    )?;

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

    // 获取未完成的采集记录
    pub async fn get_uncompleted_collect_record(&self, collect_type: i32) -> Result<Option<CollectRecord>> {
        let sql = "SELECT * FROM collect_records WHERE collect_type = ?1 AND is_completed = 0 LIMIT 1";
        
        let results = self.query(
            sql,
            &[&collect_type],
            |row| CollectRecord::from_row(row)
        )?;

        Ok(results.into_iter().next())
    }

    // 更新采集进度
    pub async fn update_collect_progress(
        &self,
        collect_type: i32,
        processed_count: i32,
        total_count: i32,
        current_page: i32
    ) -> Result<()> {
        let updated_at = get_current_date();
        let sql = "UPDATE collect_records 
                  SET processed_count = ?1, 
                      total_count = ?2,
                      current_page = ?3,
                      updated_at = ?4
                  WHERE collect_type = ?5 AND is_completed = 0";
        
        self.execute(
            sql,
            &[
                &processed_count as &dyn rusqlite::ToSql,
                &total_count,
                &current_page,
                &updated_at,
                &collect_type
            ],
        )?;

        Ok(())
    }

    // 更新采集页码
    pub async fn update_collect_page(&self, collect_type: i32, current_page: i32) -> Result<()> {
        let updated_at = get_current_date();
        let sql = "UPDATE collect_records SET current_page = ?1, updated_at = ?2 WHERE collect_type = ?3 AND is_completed = 0";
        
        self.execute(
            sql,
            &[
                &current_page as &dyn rusqlite::ToSql,
                &updated_at,
                &collect_type
            ],
        )?;

        Ok(())
    }

    // 标记采集记录为已完成
    pub async fn complete_collect_record(&self, collect_type: i32) -> Result<()> {
        let updated_at = get_current_date();
        let sql = "UPDATE collect_records SET is_completed = 1, updated_at = ?1 WHERE collect_type = ?2";
        
        self.execute(
            sql,
            &[
                &updated_at,
                &collect_type
            ],
        )?;

        Ok(())
    }

    // 创建采集记录
    pub async fn create_collect_record(&self, record: &CollectRecord) -> Result<()> {
        let sql = "INSERT INTO collect_records (collect_type, current_page, total_pages, processed_count, total_count, is_completed, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";

        self.execute(
            sql,
            &[
                &record.collect_type as &dyn rusqlite::ToSql,
                &record.current_page,
                &record.total_pages,
                &record.processed_count,
                &record.total_count,
                &record.is_completed,
                &record.created_at,
                &record.updated_at,
            ],
        )?;

        Ok(())
    }

    // 可根据需要扩展更多通用方法，如批量插入、事务等
}

impl CollectRecord {
    pub fn from_row(row: &rusqlite::Row) -> Result<CollectRecord, rusqlite::Error> {
        Ok(CollectRecord {
            id: row.get("id").ok(),
            collect_type: row.get("collect_type")?,
            current_page: row.get("current_page")?,
            total_pages: row.get("total_pages")?,
            processed_count: row.get("processed_count")?,
            total_count: row.get("total_count")?,
            is_completed: row.get("is_completed")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
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
