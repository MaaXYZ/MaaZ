use std::sync::Arc;

use maa_framework::{
    controller::{self, adb::MaaAdbControllerType},
    toolkit::{AdbDeviceInfo, MaaToolkit},
    MaaStatus,
};
use tauri::State;

use crate::{ControllerInstance, Instance, MaaZError, MaaZResult};

#[tauri::command]
pub async fn find_devices(toolkit: State<'_, MaaToolkit>) -> MaaZResult<Vec<AdbDeviceInfo>> {
    let devices = toolkit.find_adb_device()?;
    Ok(devices)
}

#[tauri::command]
pub async fn connect_to_device(
    inst: State<'_, Arc<Instance>>,
    device: AdbDeviceInfo,
    controller: State<'_, Arc<ControllerInstance>>,
) -> MaaZResult<()> {
    let agent_path = "MaaAgentBinary";

    let controller_type = MaaAdbControllerType {
        touch_type: controller::adb::MaaAdbControllerTouchType::MaaTouch,
        key_type: controller::adb::MaaAdbControllerKeyType::MaaTouch,
        screencap_type: controller::adb::MaaAdbControlScreencapType::MinicapDirect,
    };

    let controller_instance = controller::MaaControllerInstance::new_adb(
        &device.adb_path,
        &device.adb_serial,
        controller_type,
        &device.adb_config,
        agent_path,
        None,
    );

    let mut controller = controller.lock().await;

    let connection = controller_instance.post_connect();

    let ret = controller_instance.wait(connection)?;

    if let MaaStatus::Success = ret {
        inst.bind_controller(&controller_instance)?;
        *controller = Some(controller_instance);
        Ok(())
    } else {
        Err(MaaZError::ConnectionError)
    }
}
