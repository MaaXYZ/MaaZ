#![allow(clippy::needless_pass_by_value)]
// Strange since this seems to be fixed already. https://github.com/rust-lang/rust-clippy/issues/4980
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::used_underscore_binding)]
pub mod config;
pub mod device;
pub mod plugin;

use crate::{error::MaaResult, maa};

#[tauri::command]
pub fn init_maa() -> MaaResult<()> {
    maa::init_toolkit()
}