// src/app_handle.rs

use std::sync::Mutex;
use once_cell::sync::OnceCell;
use tauri::AppHandle;

static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::new();

pub fn set_app_handle(handle: AppHandle) {
    APP_HANDLE.set(Mutex::new(handle)).expect("AppHandle 只能初始化一次");
}

pub fn get_app_handle() -> &'static Mutex<AppHandle> {
    APP_HANDLE.get().expect("AppHandle 未初始化")
}