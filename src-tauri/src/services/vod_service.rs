use crate::db::modes::VideoInfo;
use crate::db::sqlite::DB_INSTANCE;
use rusqlite::Result;

#[derive(serde::Deserialize)]
pub struct VodQuery {
    pub vod_id: Option<String>,
    pub vod_type_id: Option<String>,
    pub title: Option<String>,
    pub type_name: Option<String>,
    pub year: Option<String>,
    pub language: Option<String>,
    pub actor: Option<String>,
    pub area: Option<String>,
    pub keyword: Option<String>,
    pub page: usize,
    pub page_size: usize,
}

#[derive(serde::Serialize)]
pub struct QueryResult {
    pub data: Vec<VideoInfo>,
    pub total: usize,
    pub page_size: usize,
    pub pager_count: usize,
}

#[tauri::command]
pub async fn query_videos(query: VodQuery) -> Result<QueryResult, String> {
    let mut sql = String::from("SELECT * FROM video_info WHERE 1=1");
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    let mut count_sql = String::from("SELECT COUNT(*) FROM video_info WHERE 1=1");
    let mut count_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(ref v) = query.vod_id {
        sql.push_str(" AND vod_id = ?");
        params.push(Box::new(v.clone()));
        count_sql.push_str(" AND vod_id = ?");
        count_params.push(Box::new(v.clone()));
    }
    if let Some(ref v) = query.vod_type_id {
        sql.push_str(" AND vod_type_id = ?");
        params.push(Box::new(v.clone()));
        count_sql.push_str(" AND vod_type_id = ?");
        count_params.push(Box::new(v.clone()));
    }
    if let Some(ref v) = query.title {
        sql.push_str(" AND title LIKE ?");
        params.push(Box::new(format!("%{}%", v)));
        count_sql.push_str(" AND title LIKE ?");
        count_params.push(Box::new(format!("%{}%", v)));
    }

    if let Some(ref v) = query.type_name {
        println!(" query.type_name: {:?}", v);
        sql.push_str(" AND type_name LIKE ?");
        params.push(Box::new(format!("%{}%", v)));

        count_sql.push_str(" AND type_name LIKE ?");
        count_params.push(Box::new(format!("%{}%", v)));
    }
    if let Some(ref v) = query.year {
        sql.push_str(" AND year = ?");
        params.push(Box::new(v.clone()));
        count_sql.push_str(" AND year = ?");
        count_params.push(Box::new(v.clone()));
    }
    if let Some(ref v) = query.language {
        sql.push_str(" AND language = ?");
        params.push(Box::new(v.clone()));
        count_sql.push_str(" AND language = ?");
        count_params.push(Box::new(v.clone()));
    }
    if let Some(ref v) = query.actor {
        sql.push_str(" AND actor LIKE ?");
        params.push(Box::new(format!("%{}%", v)));
        count_sql.push_str(" AND actor LIKE ?");
        count_params.push(Box::new(format!("%{}%", v)));
    }
    if let Some(ref v) = query.area {
        sql.push_str(" AND area = ?");
        params.push(Box::new(v.clone()));
        count_sql.push_str(" AND area = ?");
        count_params.push(Box::new(v.clone()));
    }
    // 新增：支持keyword多字段模糊搜索
    if let Some(ref keyword) = query.keyword {
        sql.push_str(" AND (title LIKE ? OR actor LIKE ? OR type_name LIKE ?)");
        params.push(Box::new(format!("%{}%", keyword)));
        params.push(Box::new(format!("%{}%", keyword)));
        params.push(Box::new(format!("%{}%", keyword)));

        count_sql.push_str(" AND (title LIKE ? OR actor LIKE ? OR type_name LIKE ?)");
        count_params.push(Box::new(format!("%{}%", keyword)));
        count_params.push(Box::new(format!("%{}%", keyword)));
        count_params.push(Box::new(format!("%{}%", keyword)));
    }

    sql.push_str(" ORDER BY year DESC, updated_at DESC, id DESC LIMIT ? OFFSET ?");
    params.push(Box::new(query.page_size as i64));
    params.push(Box::new(((query.page - 1) * query.page_size) as i64));
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|b| b.as_ref()).collect();
    let count_param_refs: Vec<&dyn rusqlite::ToSql> =
        count_params.iter().map(|b| b.as_ref()).collect();
    let total: usize = DB_INSTANCE
        .conn
        .lock()
        .unwrap()
        .query_row(&count_sql, count_param_refs.as_slice(), |row| {
            row.get::<_, i64>(0)
        })
        .map(|v| v as usize)
        .map_err(|e| e.to_string())?;
    let videos = DB_INSTANCE
        .query(&sql, &param_refs, |row| VideoInfo::from_row(row))
        .map_err(|e| e.to_string())?;
    let pager_count = if query.page_size > 0 {
        (total + query.page_size - 1) / query.page_size
    } else {
        0
    };
    Ok(QueryResult {
        data: videos,
        total,
        page_size: query.page_size,
        pager_count,
    })
}
