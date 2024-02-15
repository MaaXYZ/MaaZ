#![allow(clippy::multiple_unsafe_ops_per_block)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::absolute_paths)]
#![allow(unused)]

mod internal {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    unsafe impl Send for MaaInstanceAPI {}
}

use crate::{
    callback::CallbackHandler,
    error::{MaaError, MaaResult},
    model::DeviceInfo,
    InstHandle,
};
#[allow(clippy::wildcard_imports)]
use internal::*;
use serde_json::json;
use std::{
    ffi::c_void,
    mem,
    ptr::{null, null_mut},
};
use tauri::AppHandle;
use tracing::{error, event, info, trace, trace_span, Level};
use uuid::Uuid;

pub use internal::MaaInstanceAPI;

pub fn get_version() -> MaaResult<String> {
    trace!("Getting Maa version");

    let version = unsafe { MaaVersion() };
    let version = unsafe { std::ffi::CStr::from_ptr(version) };
    let version = version.to_str()?;
    Ok(version.to_owned())
}

pub fn init_toolkit() -> MaaResult<()> {
    let span = trace_span!("Initialize Maa");
    let _guard = span.enter();

    let init_ret = unsafe { MaaToolkitInit() };

    if init_ret != 1 {
        error!("MaaToolkitInit returned {}", init_ret);
        return Err(MaaError::MaaToolkitInitError);
    }

    Ok(())
}

pub fn find_devices() -> MaaResult<Vec<DeviceInfo>> {
    let post_find_device_ret = unsafe { MaaToolkitPostFindDevice() };

    if post_find_device_ret != 1 {
        error!("MaaToolkitPostFindDevice returned {}", post_find_device_ret);
        return Err(MaaError::FindDeviceError);
    }

    let device_count = unsafe { MaaToolkitWaitForFindDeviceToComplete() };
    info!("Found {} devices", device_count);

    let ret = (0..device_count)
        .map(|index| {
            trace!("Getting device info for index {}", index);

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
        .collect();

    Ok(ret)
}

pub fn get_maa_handle(app: AppHandle) -> MaaInstanceHandle {
    let span = trace_span!("Creating Maa handle");
    let _guard = span.enter();
    let handler = CallbackHandler::new(app);

    let callback_arg = Box::into_raw(Box::new(handler)).cast::<c_void>();

    trace!("Creating Maa handle");
    unsafe { MaaCreate(Some(callback_fn), callback_arg) }
}

pub fn init_resources(maa_handle: &InstHandle, resources: &[String]) -> MaaResult<()> {
    let span = trace_span!("Initialize Maa resources");
    let _guard = span.enter();
    let resource_handle = unsafe { MaaResourceCreate(None, null_mut()) };
    let ret = resources
        .iter()
        .map(|resource| {
            let resource_dir = format!("resources/{resource}");
            to_cstring(&resource_dir)
        })
        .map(|resource_dir| unsafe { MaaResourcePostPath(resource_handle, resource_dir) })
        .map(|resource_id| unsafe { MaaResourceWait(resource_handle, resource_id) })
        .map(|resource_ret| {
            if resource_ret != MaaStatusEnum_MaaStatus_Success {
                error!("Maa resource wait returned {}", resource_ret);
                return Err(MaaError::ResourceInitError);
            }
            Ok(())
        })
        .all(|x| x.is_ok());

    if !ret {
        return Err(MaaError::ResourceInitError);
    }

    trace!("Binding Maa resources");
    let bind_ret = unsafe { MaaBindResource(maa_handle.0, resource_handle) };
    trace!("Maa resources initialized");

    if bind_ret != 1 {
        error!("Maa bind resource returned {}", bind_ret);
        return Err(MaaError::ResourceBindError);
    }

    Ok(())
}

unsafe extern "C" fn callback_fn(
    msg: MaaStringView,
    details_json: MaaStringView,
    handler: MaaTransparentArg,
) {
    trace!("Callback received");
    let handler = handler.cast::<CallbackHandler>();

    let msg = maa_string_view_to_string(msg);
    let details = maa_string_view_to_string(details_json);

    event!(Level::TRACE, msg=%msg, details=%details);

    #[allow(clippy::unwrap_used)]
    handler.as_ref().unwrap().handle_callback(msg, details);
}

pub fn connect_to_device(handle: &InstHandle, device_info: &DeviceInfo) -> u8 {
    let span = trace_span!("Connect to device");
    let _guard = span.enter();

    info!(device_name=%device_info.name, adb_serial=%device_info.adb_serial, adb_path=%device_info.adb_path, adb_config=%device_info.adb_config);

    let adb_path = to_cstring(&device_info.adb_path);
    let address = to_cstring(&device_info.adb_serial);
    let controller_type = MaaAdbControllerTypeEnum_MaaAdbControllerType_Input_Preset_Maatouch
        | MaaAdbControllerTypeEnum_MaaAdbControllerType_Screencap_MinicapDirect;
    let config = to_cstring(&device_info.adb_config);
    let agent_path = to_cstring("MaaAgentBinary");

    let controller_handle = unsafe {
        MaaAdbControllerCreateV2(
            adb_path,
            address,
            controller_type,
            config,
            agent_path,
            None,
            null_mut(),
        )
    };

    trace!("Posting connection to controller");
    let ctrl_id = unsafe { MaaControllerPostConnection(controller_handle) };
    info!("Got controller id {ctrl_id}");

    trace!("Waiting for controller to connect");
    unsafe { MaaControllerWait(controller_handle, ctrl_id) };

    trace!("Binding controller to device");
    let ret = unsafe { MaaBindController(handle.0, controller_handle) };
    trace!("Controller bound to device ret: {ret}");

    ret
}

// pub fn post_task<T>(handle: &InstHandle, task_type: &TaskType, task_param: &T) -> i64
// where
//     T: TaskParam,
// {
//     let task = task_type.get_string();

//     let param = task_param.get_param();
//     let param = param.to_string();
//     trace!("Posting task");
//     info!(task=%task, param=%param);

//     unsafe { MaaPostTask(handle.0, to_cstring(&task), to_cstring(&param)) }
// }

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
