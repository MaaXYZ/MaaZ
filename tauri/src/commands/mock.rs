use maa_framework::{controller::adb::MaaAdbControllerType, toolkit::AdbDeviceInfo};

pub fn mock_adb_device() -> AdbDeviceInfo {
    AdbDeviceInfo {
        adb_path: "/usr/bin/adb".to_owned(),
        adb_serial: "emulator-5554".to_owned(),
        adb_config: String::new(),
        adb_controller_type: MaaAdbControllerType {
            touch_type: maa_framework::controller::adb::MaaAdbControllerTouchType::MaaTouch,
            key_type: maa_framework::controller::adb::MaaAdbControllerKeyType::MaaTouch,
            screencap_type: maa_framework::controller::adb::MaaAdbControlScreencapType::MinicapDirect,
        },
        name: "MOCK Device".to_owned(),
    }
}