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
    ConnectSuccess,
    ConnectFailed,
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
        let str_value = CString::new(value.clone()).map_err(|_| MaaError::Utf8Error(value.clone()))?;
        let bytes = str_value.as_bytes_with_nul();
        match bytes {
            maa::MaaMsg_Controller_Action_Completed => Ok(Self::ControllerActionCompleted),
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

        match CallbackEvent::try_from(msg.clone()) {
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
