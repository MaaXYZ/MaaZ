use tauri::State;

use crate::{
    maa::{self, DeviceInfo},
    InstHandle, MaaError, MaaResult,
};

#[tauri::command]
pub async fn find_devices() -> MaaResult<Vec<DeviceInfo>> {
    maa::find_devices()
}

#[tauri::command]
pub async fn connect_to_device(inst: State<'_, InstHandle>, device: DeviceInfo) -> MaaResult<()> {
    let ret = maa::connect_to_device(*inst, &device);
    if ret == 1 {
        Ok(())
    } else {
        Err(MaaError::DeviceConnectionError)
    }
}
