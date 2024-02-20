use std::{ffi::CString, sync::Arc};

use serde::{Deserialize, Serialize};
use tauri::{async_runtime, AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use tracing::{error, info, trace_span};

use crate::{maa, ConfigHolderState, InstHandle, MaaError, TaskQueueState};

pub const CALLBACK_EVENT: &str = "callback";
pub const QUEUE_DONE_EVENT: &str = "queue-done";

#[derive(Serialize, Deserialize)]
pub struct CallbackTriggerPayload {
    pub msg: String,
    pub data: String,
}

impl CallbackTriggerPayload {
    pub fn new(msg: String, data: String) -> Self {
        Self { msg, data }
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
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
        let str_value =
            CString::new(value.clone()).map_err(|e| MaaError::Utf8Error(e.to_string()))?;
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
            maa::MaaMsg_Controller_ResolutionGetFailed => {
                Ok(CallbackEvent::ControllerResolutionGetFailed)
            }
            maa::MaaMsg_Controller_ScreencapInited => Ok(CallbackEvent::ControllerScreencapInited),
            maa::MaaMsg_Controller_ScreencapInitFailed => {
                Ok(CallbackEvent::ControllerScreencapInitFailed)
            }
            maa::MaaMsg_Controller_TouchInputInited => {
                Ok(CallbackEvent::ControllerTouchInputInited)
            }
            maa::MaaMsg_Controller_TouchInputInitFailed => {
                Ok(CallbackEvent::ControllerTouchInputInitFailed)
            }
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
struct CallbackEventPayload {
    pub event: CallbackEvent,
    pub data: String,
}

impl CallbackEventPayload {
    pub fn new(event: CallbackEvent, data: &str) -> Self {
        Self {
            event,
            data: data.to_owned(),
        }
    }
}

macro_rules! notify {
    ($app:expr,$title:expr) => {
        match $app.notification().builder().title($title).show()
        {
            Ok(_) => {}
            Err(e) => {
                error!("Error while showing notification: {}", e);
            }
        }
    };

    ($app:expr,$title:expr,$body:expr) => {
        match $app.notification().builder()
            .title($title)
            .body($body)
            .show()
        {
            Ok(_) => {}
            Err(e) => {
                error!("Error while showing notification: {}", e);
            }
        }
    };
}

pub fn setup_callback(
    app: &AppHandle,
    queue: TaskQueueState,
    config: ConfigHolderState,
    handle: InstHandle,
) {
    let app_handle = app.clone();

    app.listen(CALLBACK_EVENT, move |event| {
        let span = trace_span!("callback");
        let _guard = span.enter();

        let payload_string = event.payload();
        info!("Received callback payload: {}", payload_string);
        let Ok(payload) = serde_json::from_str::<CallbackTriggerPayload>(payload_string)
            .inspect_err(|e| {
                error!("Error while deserializing callback payload: {}", e);
            })
        else {
            return;
        };

        let Ok(callback_event) = CallbackEvent::try_from(payload.msg).inspect_err(|e| {
            error!("Error while converting callback event: {:?}", e);
        }) else {
            return;
        };

        info!("Emitting callback event: {:?}", callback_event);
        let payload = CallbackEventPayload::new(callback_event, &payload.data);
        #[allow(clippy::unwrap_used)]
        app_handle.emit(CALLBACK_EVENT, &payload).unwrap();

        let app_handle = app_handle.clone();

        #[allow(clippy::match_same_arms)]
        match callback_event {
            CallbackEvent::TaskCompleted => {
                let queue = Arc::clone(&queue);
                let config = Arc::clone(&config);
                async_runtime::spawn(async move {
                    info!("Running next task");
                    let mut queue = queue.lock().await;
                    let config = config.lock().await;
                    let has_next = queue.run_next(handle, config.config());
                    if !has_next {
                        #[allow(clippy::unwrap_used)]
                        app_handle.emit(QUEUE_DONE_EVENT, ()).unwrap();
                        notify!(app_handle, "Task Queue Finished");
                    }
                });
            }
            CallbackEvent::TaskFailed => {
                notify!(app_handle, "Task Failed", &payload.data);
            }
            CallbackEvent::Invalid => {}
            CallbackEvent::ResourceStartLoading => {}
            CallbackEvent::ResourceLoadingCompleted => {}
            CallbackEvent::ResourceLoadingFailed => {}
            CallbackEvent::ControllerUUIDGot => {}
            CallbackEvent::ControllerUUIDGetFailed => {}
            CallbackEvent::ControllerResolutionGot => {}
            CallbackEvent::ControllerResolutionGetFailed => {}
            CallbackEvent::ControllerScreencapInited => {}
            CallbackEvent::ControllerScreencapInitFailed => {}
            CallbackEvent::ControllerTouchInputInited => {}
            CallbackEvent::ControllerTouchInputInitFailed => {}
            CallbackEvent::ControllerActionStarted => {}
            CallbackEvent::ControllerActionCompleted => {}
            CallbackEvent::ControllerActionFailed => {}
            CallbackEvent::ControllerConnectSuccess => {}
            CallbackEvent::ControllerConnectFailed => {}
            CallbackEvent::TaskStarted => {}
            CallbackEvent::TaskStopped => {}
            CallbackEvent::TaskFocusHit => {}
            CallbackEvent::TaskFocusRunout => {}
            CallbackEvent::TaskFocusCompleted => {}
        }
    });
}
