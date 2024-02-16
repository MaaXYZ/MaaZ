use serde::Serialize;
use tauri::{AppHandle, Manager};

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
    ConnectSuccess,
    ConnectFailed,
    ActionStarted,
    ActionCompleted,
    ActionFailed,
    TaskStarted,
    TaskCompleted,
    TaskFailed,
    TaskStopped,
    TaskFocusHit,
    TaskFocusRunout,
    TaskFocusCompleted,
}

impl From<String> for CallbackEvent {
    fn from(value: String) -> Self {
        unimplemented!()
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
        let payload = CallbackPayload {
            event: msg.into(),
            details,
        };
        #[allow(clippy::unwrap_used)]
        self.app.emit_all("callback", payload).unwrap();
    }
}
