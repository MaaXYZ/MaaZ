use tauri::State;

use crate::{
    error::{MaaError, MaaResult},
    maa,
    model::DeviceInfo,
    InstHandle,
};

#[tauri::command]
pub fn init_devices() -> Vec<DeviceInfo> {
    maa::init()
}

#[tauri::command]
pub async fn connect_to_device(inst: State<'_, InstHandle>, device: DeviceInfo) -> MaaResult<()> {
    let ret = maa::connect_to_device(&inst, &device);
    if ret == 1 {
        Ok(())
    } else {
        Err(MaaError::DeviceConnectionError)
    }
}
