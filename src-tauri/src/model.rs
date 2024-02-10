use serde::Serialize;


#[derive(Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub adb_config: String,
    pub adb_serial: String,
    pub adb_path: String,
}