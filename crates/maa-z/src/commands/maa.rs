use tauri::State;

use crate::{error::MaaResult, maa, InstHandle};

#[tauri::command]
pub fn init_maa(inst: State<'_, InstHandle>) -> MaaResult<()> {
    maa::init_toolkit()?;
    maa::init_resources(&inst)
}
