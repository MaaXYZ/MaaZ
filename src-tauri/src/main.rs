// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::MaaResult;
use maa::MaaInstanceAPI;
use model::DeviceInfo;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tauri::State;

use crate::error::MaaError;

mod error;
mod maa;
mod model;

#[derive(Clone)]
pub struct InstHandle(*mut MaaInstanceAPI);

// Safety: InstHandle is Send and Sync because MaaInstanceAPI is Send and Sync
unsafe impl Send for InstHandle {}
// Safety: InstHandle is Send and Sync because MaaInstanceAPI is Send and Sync
unsafe impl Sync for InstHandle {}

fn main() {
    let _guard = init_tracing();

    tracing::info!("Starting Maa");

    let handle = maa::get_maa_handle();
    let inst = InstHandle(handle);

    let inst_clone = inst.clone();

    tauri::async_runtime::spawn(async move {
        maa::init_resources(&inst_clone);
    });

    #[allow(clippy::expect_used)]
    #[allow(clippy::str_to_string)]
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .manage(inst)
        .invoke_handler(tauri::generate_handler![init_devices,connect_to_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(clippy::absolute_paths)]
fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "maa.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .init();

    guard
}

#[tauri::command]
fn init_devices() -> Vec<DeviceInfo> {
    maa::init()
}

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
fn connect_to_device(inst: State<'_, InstHandle>, device: DeviceInfo) -> MaaResult<()> {
    let ret = maa::connect_to_device(&inst, &device);
    if ret == 1 {
        Ok(())
    } else {
        Err(MaaError::DeviceConnectionError)
    }
}