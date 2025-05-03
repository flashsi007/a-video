use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::Local;
use crate::structs::structs::{LogType,Logger};
use crate::app_handle::get_app_handle;  
use tauri::Manager;

impl LogType {
    /// 获取日志类型的字符串表示
    fn as_str(&self) -> &'static str {
        match self {
            LogType::Info => "INFO",
            LogType::Error => "ERROR",
            LogType::Success => "SUCCESS",
            LogType::Warning => "WARNING",
        }
    }

    /// 获取日志类型对应的控制台颜色代码
    fn color_code(&self) -> &'static str {
        match self {
            LogType::Info => "\u{001B}[36m", // 青色
            LogType::Error => "\u{001B}[31m", // 红色
            LogType::Success => "\u{001B}[32m", // 绿色
            LogType::Warning => "\u{001B}[33m", // 黄色
        }
    }
}



impl Logger {
    /// 创建一个新的日志记录器实例
    pub fn new(log_file_path: &str) -> Self {
        // 确保日志目录存在
        if let Some(parent) = Path::new(log_file_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create log directory");
            }
        }

        Logger {
            log_file_path: log_file_path.to_string(),
        }
    }

    /// 记录日志
    pub fn log(&self, log_type: LogType, message: &str) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_type_str = log_type.as_str();
        let log_message = format!("[{}] [{}] {}", now, log_type_str, message);

        // 写入日志文件
        self.write_to_file(&log_message);

        // 生产环境不需要控制台输出
        // let color_code = log_type.color_code();
        // let reset_code = "\u{001B}[0m";
        // println!("{}{}{}", color_code, log_message, reset_code);
    }

    /// 记录信息日志
    pub fn info(&self, message: &str) {
        self.log(LogType::Info, message);
    }

    /// 记录错误日志
    pub fn error(&self, message: &str) {
        self.log(LogType::Error, message);
    }

    /// 记录成功日志
    pub fn success(&self, message: &str) {
        self.log(LogType::Success, message);
    }

    /// 记录警告日志
    pub fn warning(&self, message: &str) {
        self.log(LogType::Warning, message);
    }

    /// 将日志写入文件
    fn write_to_file(&self, message: &str) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file_path);

        match file {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", message) {
                    eprintln!("Failed to write to log file: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
            }
        }
    }
}


fn get_log_path() -> Result<String, rusqlite::Error> {
    let app_handle = get_app_handle().lock().map_err(|_| rusqlite::Error::InvalidPath("AppHandle lock failed".to_string().into()))?;
    let log_dir = app_handle.path().resource_dir().map_err(|_| rusqlite::Error::InvalidPath("Resource dir not found".to_string().into()))?;
    // path.push("video.db");
    let log_file = log_dir.join(format!("logs/{}.log", Local::now().format("%Y-%m-%d")));
    Ok(log_file.to_str().unwrap().to_string())
}

/// 创建默认日志记录器的便捷函数
pub fn create_default_logger() -> Logger {  
  let log_file  = get_log_path();
    
    
    Logger::new(log_file.unwrap().as_str())
}

/// 全局日志函数 - 信息
pub fn info(message: &str) {
    let logger = create_default_logger();
    logger.info(message);
}

/// 全局日志函数 - 错误
pub fn error(message: &str) {
    let logger = create_default_logger();
    logger.error(message);
}

/// 全局日志函数 - 成功
pub fn success(message: &str) {
    let logger = create_default_logger();
    logger.success(message);
}

/// 全局日志函数 - 警告
pub fn warning(message: &str) {
    let logger = create_default_logger();
    logger.warning(message);
}