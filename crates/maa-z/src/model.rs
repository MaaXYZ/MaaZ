use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub adb_config: String,
    pub adb_serial: String,
    pub controller_type: i32,
    pub adb_path: String,
}
