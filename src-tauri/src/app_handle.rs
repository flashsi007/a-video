// src/app_handle.rs

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::new();
// static  APP_HANDLE_DIR: OnceCell<Mutex<std::path::PathBuf>> = OnceCell::new();
pub fn set_app_handle(handle: AppHandle) {
    APP_HANDLE
        .set(Mutex::new(handle))
        .expect("AppHandle 只能初始化一次"); 

       

     
}

pub fn get_app_handle() -> &'static Mutex<AppHandle> {
    APP_HANDLE.get().expect("AppHandle 未初始化")
}


 