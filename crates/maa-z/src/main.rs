// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use maa::MaaInstanceAPI;
use queue::TaskQueue;
use serde::Serialize;
use tauri::{async_runtime::Mutex, Manager};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::callback::setup_callback;

mod callback;
mod commands;
mod config;
mod maa;
mod queue;
mod task;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct InstHandle(*mut MaaInstanceAPI);

// Safety: InstHandle is Send and Sync because MaaInstanceAPI is Send and Sync
unsafe impl Send for InstHandle {}
// Safety: InstHandle is Send and Sync because MaaInstanceAPI is Send and Sync
unsafe impl Sync for InstHandle {}

pub type ConfigHolderState = Arc<Mutex<config::ConfigHolder>>;

pub type TaskQueueState = Arc<Mutex<TaskQueue>>;

fn main() {
    let _guard = init_tracing();

    tracing::info!("Starting Maa");

    #[allow(clippy::expect_used)]
    #[allow(clippy::str_to_string)]
    tauri::Builder::default()
        .setup(|app| {
            let path =
                std::env::current_exe().expect("error while getting current executable path");
            let config_file = path.with_file_name("maa.toml");
            let config =
                config::ConfigHolder::new(config_file).expect("error while reading config file");
            let config = Arc::new(Mutex::new(config));
            app.manage(Arc::clone(&config));

            let task_queue = TaskQueueState::default();
            app.manage(Arc::clone(&task_queue));

            let handle = maa::get_maa_handle(app.app_handle());
            let inst = InstHandle(handle);

            app.manage(inst);

            setup_callback(
                &app.app_handle(),
                Arc::clone(&task_queue),
                Arc::clone(&config),
                inst,
            );

            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::device::find_devices,
            commands::device::connect_to_device,
            commands::config::get_config,
            commands::config::set_client_type,
            commands::task::add_task_to_queue,
            commands::task::start_queue,
            commands::task::stop_queue,
            commands::task::remove_from_queue,
            commands::task::get_queue,
            commands::init_maa,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(clippy::absolute_paths)]
fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "maa.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    #[allow(clippy::expect_used)]
    let timer = OffsetTime::local_rfc_3339().expect("error while creating timer");

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .with_timer(timer)
        .init();

    guard
}

pub type MaaResult<T> = Result<T, MaaError>;

#[derive(Serialize, Debug)]
pub enum MaaError {
    Utf8Error(String),
    MaaHandleInitError,
    DeviceConnectionError,
    IOError(String),
    TOMLDeError(String),
    TOMLSerError(String),
    UnknowTaskError(String),
    ResourceInitError,
    ResourceBindError,
    FindDeviceError,
    MaaToolkitInitError,
    InvalidCallbackEvent(String),
    StopTaskError,
    QueueDidnotStart,
}

impl From<std::str::Utf8Error> for MaaError {
    fn from(e: std::str::Utf8Error) -> Self {
        MaaError::Utf8Error(e.to_string())
    }
}

impl From<std::io::Error> for MaaError {
    fn from(e: std::io::Error) -> Self {
        MaaError::IOError(e.to_string())
    }
}

impl From<toml::de::Error> for MaaError {
    fn from(e: toml::de::Error) -> Self {
        MaaError::TOMLDeError(e.to_string())
    }
}

impl From<toml::ser::Error> for MaaError {
    fn from(e: toml::ser::Error) -> Self {
        MaaError::TOMLSerError(e.to_string())
    }
}
