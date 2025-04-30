use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

// 👇 必须引入 Manager Trait 才能使用 `.path()`
use tauri::Manager;

#[derive(Debug)]
pub enum ResourceError {
    DirNotFound,
    FileNotFound(String),
    IoError(String),
    JsonError(String),
}

impl From<std::io::Error> for ResourceError {
    fn from(e: std::io::Error) -> Self {
        ResourceError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for ResourceError {
    fn from(e: serde_json::Error) -> Self {
        ResourceError::JsonError(e.to_string())
    }
}

// 获取资源文件路径
pub fn get_resource_path(app_handle: &AppHandle, relative_path: &str) -> Result<PathBuf, ResourceError> {
    let mut path = app_handle
        .path()
        .resource_dir()
        .map_err(|_| ResourceError::DirNotFound)?; // ✅ 替代之前的 ok_or()

    path.push(relative_path);

    if !path.exists() {
        return Err(ResourceError::FileNotFound(relative_path.to_string()));
    }

    Ok(path)
}

// 读取文本文件
pub fn read_resource_file(app_handle: &AppHandle, relative_path: &str) -> Result<String, ResourceError> {
    let path = get_resource_path(app_handle, relative_path)?;
    fs::read_to_string(path).map_err(Into::into)
}

// 读取并解析 JSON 文件
pub fn read_resource_json<T>(app_handle: &AppHandle, relative_path: &str) -> Result<T, ResourceError>
where
    T: DeserializeOwned,
{
    let content = read_resource_file(app_handle, relative_path)?;
    serde_json::from_str(&content).map_err(Into::into)
}