#![allow(clippy::needless_pass_by_value)]
// Strange since this seems to be fixed already. https://github.com/rust-lang/rust-clippy/issues/4980
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::used_underscore_binding)]

use tauri::State;

use crate::{maa, InstHandle, MaaResult};
pub mod config;
pub mod device;
pub mod task;

#[tauri::command]
pub async fn init_maa(inst: State<'_, InstHandle>) -> MaaResult<()> {
    maa::init_toolkit()?;
    maa::init_resources(*inst)
}
