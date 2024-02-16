use tauri::{AppHandle, Manager};

pub struct CallbackHandler {
    app: AppHandle,
}

enum CallbackEvent {
    TaskSuccess,
    TaskStart
}

struct CallbackPayload {

}

impl CallbackHandler {
    pub fn new(app: AppHandle) -> CallbackHandler {
        CallbackHandler { app }
    }

    pub fn handle_callback(&self, msg: String, details: String) {
        #[allow(clippy::unwrap_used)]
        self.app.emit_all("callback", details).unwrap();
    }
}
