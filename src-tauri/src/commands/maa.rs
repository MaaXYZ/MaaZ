use tauri::State;

use crate::{error::MaaResult, maa, InstHandle};

#[tauri::command]
pub fn init_resources(inst: State<'_, InstHandle>) -> MaaResult<()> {
    maa::init_resources(&inst)
}
