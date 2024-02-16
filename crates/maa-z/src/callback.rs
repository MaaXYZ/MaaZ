use std::ffi::CString;

use serde::Serialize;
use tauri::{AppHandle, Manager};
use tracing::error;

use crate::{error::MaaError, maa};

pub struct CallbackHandler {
    app: AppHandle,
}

#[derive(Serialize, Clone)]
enum CallbackEvent {
    Invalid,
    ResourceStartLoading,
    ResourceLoadingCompleted,
    ResourceLoadingFailed,
    ControllerUUIDGot,
    ControllerUUIDGetFailed,
    ControllerResolutionGot,
    ControllerResolutionGetFailed,
    ControllerScreencapInited,
    ControllerScreencapInitFailed,
    ControllerTouchInputInited,
    ControllerTouchInputInitFailed,
    ControllerActionStarted,
    ControllerActionCompleted,
    ControllerActionFailed,
    ControllerConnectSuccess,
    ControllerConnectFailed,
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    TaskStopped,
    TaskFocusHit,
    TaskFocusRunout,
    TaskFocusCompleted,
}

impl TryFrom<String> for CallbackEvent {
    type Error = MaaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let str_value = CString::new(value.clone()).map_err(|e| MaaError::Utf8Error(e.to_string()))?;
        let bytes = str_value.as_bytes_with_nul();
        match bytes {
            // generate all arms
            maa::MaaMsg_Invalid => Ok(CallbackEvent::Invalid),
            maa::MaaMsg_Resource_StartLoading => Ok(CallbackEvent::ResourceStartLoading),
            maa::MaaMsg_Resource_LoadingCompleted => Ok(CallbackEvent::ResourceLoadingCompleted),
            maa::MaaMsg_Resource_LoadingFailed => Ok(CallbackEvent::ResourceLoadingFailed),
            maa::MaaMsg_Controller_UUIDGot => Ok(CallbackEvent::ControllerUUIDGot),
            maa::MaaMsg_Controller_UUIDGetFailed => Ok(CallbackEvent::ControllerUUIDGetFailed),
            maa::MaaMsg_Controller_ResolutionGot => Ok(CallbackEvent::ControllerResolutionGot),
            maa::MaaMsg_Controller_ResolutionGetFailed => Ok(CallbackEvent::ControllerResolutionGetFailed),
            maa::MaaMsg_Controller_ScreencapInited => Ok(CallbackEvent::ControllerScreencapInited),
            maa::MaaMsg_Controller_ScreencapInitFailed => Ok(CallbackEvent::ControllerScreencapInitFailed),
            maa::MaaMsg_Controller_TouchInputInited => Ok(CallbackEvent::ControllerTouchInputInited),
            maa::MaaMsg_Controller_TouchInputInitFailed => Ok(CallbackEvent::ControllerTouchInputInitFailed),
            maa::MaaMsg_Controller_Action_Started => Ok(CallbackEvent::ControllerActionStarted),
            maa::MaaMsg_Controller_Action_Completed => Ok(CallbackEvent::ControllerActionCompleted),
            maa::MaaMsg_Controller_Action_Failed => Ok(CallbackEvent::ControllerActionFailed),
            maa::MaaMsg_Controller_ConnectSuccess => Ok(CallbackEvent::ControllerConnectSuccess),
            maa::MaaMsg_Controller_ConnectFailed => Ok(CallbackEvent::ControllerConnectFailed),
            maa::MaaMsg_Task_Started => Ok(CallbackEvent::TaskStarted),
            maa::MaaMsg_Task_Completed => Ok(CallbackEvent::TaskCompleted),
            maa::MaaMsg_Task_Failed => Ok(CallbackEvent::TaskFailed),
            maa::MaaMsg_Task_Stopped => Ok(CallbackEvent::TaskStopped),
            maa::MaaMsg_Task_Focus_Hit => Ok(CallbackEvent::TaskFocusHit),
            maa::MaaMsg_Task_Focus_Runout => Ok(CallbackEvent::TaskFocusRunout),
            maa::MaaMsg_Task_Focus_Completed => Ok(CallbackEvent::TaskFocusCompleted),

            _ => Err(MaaError::InvalidCallbackEvent(value)),
        }
    }

}

#[derive(Serialize, Clone)]
struct CallbackPayload {
    event: CallbackEvent,
    details: String,
}

impl CallbackHandler {
    pub fn new(app: AppHandle) -> CallbackHandler {
        CallbackHandler { app }
    }

    pub fn handle_callback(&self, msg: String, details: String) {

        match CallbackEvent::try_from(msg) {
            Ok(event) => {
                let payload = CallbackPayload {
                    event,
                    details,
                };
                #[allow(clippy::unwrap_used)]
                self.app.emit_all("callback", payload).unwrap();
            }
            Err(e) => {
                error!("Error while converting callback event: {:?}", e);
            }
        }
    }
}
