#![allow(clippy::needless_pass_by_value)]
// Strange since this seems to be fixed already. https://github.com/rust-lang/rust-clippy/issues/4980
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::used_underscore_binding)]

use tauri::{AppHandle, State, Window};

use crate::{maa, InstHandle, MaaResult};
pub mod config;
pub mod device;
pub mod task;

#[tauri::command]
pub async fn init_maa(inst: State<'_, InstHandle>) -> MaaResult<()> {
    maa::init_toolkit()?;
    maa::init_resources(*inst)
}

#[tauri::command]
#[allow(clippy::unused_async)]
// This needs to be async otherwise it will cause deadlock on windows
pub async fn start_mini_window(app: AppHandle) -> MaaResult<()> {
    tauri::WebviewWindowBuilder::new(&app, "mini", tauri::WebviewUrl::App("mini.html".into()))
        .title("MaaZ Mini")
        .inner_size(256.0, 512.0)
        .decorations(false)
        .maximizable(false)
        .build()?
        .show()?;

    Ok(())
}

#[tauri::command]
pub async fn set_window_on_top(window:Window, on_top: bool) -> MaaResult<()> {
    window.set_always_on_top(on_top)?;
    Ok(())
}