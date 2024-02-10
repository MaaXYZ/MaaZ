// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use model::DeviceInfo;
use tauri::async_runtime::Mutex;

mod maa;
mod model;

fn main() {

    // SAFETY: This is a safe pointer dereference
    let handle = unsafe { *maa::get_maa_handle() };

    #[allow(clippy::expect_used)]
    #[allow(clippy::str_to_string)]
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(handle)))
        .invoke_handler(tauri::generate_handler![init_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_devices() -> Vec<DeviceInfo> {
    maa::init()
}