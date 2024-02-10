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

use std::ptr::null_mut;
use internal::*;
use crate::model::DeviceInfo;

pub fn get_version() -> String {
    let version = unsafe { MaaVersion() };
    let version = unsafe { std::ffi::CStr::from_ptr(version) };
    let version = version.to_str().unwrap();
    version.to_string()
}

pub fn init() -> Vec<DeviceInfo> {
    unsafe {
        MaaToolkitInit();
        MaaToolkitPostFindDevice()
    };

    let device_count = unsafe { MaaToolkitWaitForFindDeviceToComplete() };

    (0..device_count).map(|index| {
        let device_name = unsafe { MaaToolkitGetDeviceName(index) };
        let device_name = maa_string_view_to_string(device_name);

        let adb_config = unsafe { MaaToolkitGetDeviceAdbConfig(index) };
        let adb_config = maa_string_view_to_string(adb_config);

        let adb_serial = unsafe { MaaToolkitGetDeviceAdbSerial(index) };
        let adb_serial = maa_string_view_to_string(adb_serial);

        let adb_path = unsafe { MaaToolkitGetDeviceAdbPath(index) };
        let adb_path = maa_string_view_to_string(adb_path);

        DeviceInfo {
            name: device_name,
            adb_config,
            adb_serial,
            adb_path,
        }
    }).collect()
}

pub fn get_maa_handle() -> MaaInstanceHandle {
    unsafe {
        MaaCreate(None, null_mut())
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

pub fn post_task_no_param(maa_handle: MaaInstanceHandle, task_name: &str) -> MaaTaskId {
    unsafe {
        MaaPostTask(maa_handle, to_cstring(task_name), u8_to_cstring(MaaTaskParam_Empty))
    }
}

pub fn wait_for_task(maa_handle: MaaInstanceHandle, task_id: MaaTaskId) {
    unsafe {
        MaaWaitTask(maa_handle, task_id);
    }
}