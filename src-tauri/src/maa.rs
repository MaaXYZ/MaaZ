#![allow(clippy::multiple_unsafe_ops_per_block)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::absolute_paths)]

mod internal {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    unsafe impl Send for MaaInstanceAPI {}
}

use crate::{
    error::{MaaError, MaaResult},
    model::DeviceInfo, InstHandle,
};
#[allow(clippy::wildcard_imports)]
use internal::*;
use std::ptr::null_mut;

pub use internal::MaaInstanceAPI;

pub fn get_version() -> MaaResult<String> {

    tracing::trace!("Getting Maa version");

    let version = unsafe { MaaVersion() };
    let version = unsafe { std::ffi::CStr::from_ptr(version) };
    let version = version.to_str()?;
    Ok(version.to_owned())
}

pub fn init() -> Vec<DeviceInfo> {

    tracing::trace!("Initializing Maa");

    unsafe {
        MaaToolkitInit();
        MaaToolkitPostFindDevice()
    };

    let device_count = unsafe { MaaToolkitWaitForFindDeviceToComplete() };

    (0..device_count)
        .map(|index| {

            tracing::info!("Getting device info for index {}", index);

            let device_name = unsafe { MaaToolkitGetDeviceName(index) };
            let device_name = maa_string_view_to_string(device_name);

            let adb_config = unsafe { MaaToolkitGetDeviceAdbConfig(index) };
            let adb_config = maa_string_view_to_string(adb_config);

            let adb_serial = unsafe { MaaToolkitGetDeviceAdbSerial(index) };
            let adb_serial = maa_string_view_to_string(adb_serial);

            let controller_type = unsafe { MaaToolkitGetDeviceAdbControllerType(index) };

            let adb_path = unsafe { MaaToolkitGetDeviceAdbPath(index) };
            let adb_path = maa_string_view_to_string(adb_path);

            DeviceInfo {
                name: device_name,
                adb_config,
                adb_serial,
                controller_type,
                adb_path,
            }
        })
        .collect()
}

pub fn get_maa_handle() -> MaaInstanceHandle {
    tracing::info!("Creating Maa handle");
    unsafe { MaaCreate(None, null_mut()) }
}

pub fn init_resources(maa_handle: &InstHandle) {
    tracing::trace!("Initializing Maa resources");
    let resource_handle = unsafe { MaaResourceCreate(None, null_mut()) };
    let resource_dir = to_cstring("resources");
    let resource_id = unsafe { MaaResourcePostPath(resource_handle, resource_dir) };
    let ret = unsafe { MaaResourceWait(resource_handle, resource_id) };

    tracing::debug!("Maa resource wait returned {}", ret);
    tracing::trace!("Binding Maa resources");
    unsafe { MaaBindResource(maa_handle.0, resource_handle) };
    tracing::trace!("Maa resources initialized");
}

pub fn connect_to_device(handle: &InstHandle, device_info: &DeviceInfo) -> u8 {

    tracing::info!("Connecting to device {}", device_info.name);

    let adb_path = to_cstring(&device_info.adb_path);
    let address = to_cstring(&device_info.adb_serial);
    let type_ = device_info.controller_type;
    let config = to_cstring(&device_info.adb_config);
    let agent_path = to_cstring("MaaAgentBinary");
    let callback = None;
    let callback_arg = null_mut();

    let controller_handle = unsafe {
        MaaAdbControllerCreateV2(
            adb_path,
            address,
            type_,
            config,
            agent_path,
            callback,
            callback_arg,
        )
    };

    tracing::trace!("Binding controller to device");
    let ret = unsafe { MaaBindController(handle.0, controller_handle) };

    tracing::trace!("Controller bound to device ret: {ret}");

    ret
}

pub fn check_init_state(handle: MaaInstanceHandle) -> MaaResult<()> {
    let state = unsafe { MaaInited(handle) };
    if state == 0 {
        Err(MaaError::MaaHandleInitError)
    } else {
        Ok(())
    }
}

#[allow(clippy::unwrap_used)]
#[inline]
fn to_cstring(s: &str) -> *const i8 {
    std::ffi::CString::new(s).unwrap().into_raw()
}

#[allow(clippy::unwrap_used)]
#[inline]
fn u8_to_cstring(s: &[u8]) -> *const i8 {
    std::ffi::CString::new(s).unwrap().into_raw()
}

#[allow(clippy::unwrap_used)]
#[inline]
fn maa_string_view_to_string(s: MaaStringView) -> String {
    let s = unsafe { std::ffi::CStr::from_ptr(s) };
    let s = s.to_str().unwrap();
    s.to_owned()
}
