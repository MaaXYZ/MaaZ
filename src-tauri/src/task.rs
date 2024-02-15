use serde::Serialize;
use serde_json::{json, Value};

use crate::config::start_up::StartUpConfig;

pub enum TaskType {
    StartUp,
}

impl TaskType {
    pub fn get_string(&self) -> String {
        match *self {
            TaskType::StartUp => "start_up".to_owned(),
        }
    }
}

pub trait TaskParam: Serialize {
    fn get_param(&self) -> Value;
}

#[derive(Serialize)]
pub struct StartUpParam {
    pub package: String,
}

impl TaskParam for StartUpParam {
    fn get_param(&self) -> Value {
        #[allow(clippy::unwrap_used)]
        let inner = serde_json::to_value(self).unwrap();
        json!({
            "sub_start_app": inner,
        })
    }
}

impl From<StartUpConfig> for StartUpParam {
    fn from(config: StartUpConfig) -> Self {
        Self {
            package: config.client_type.get_package_name(),
        }
    }
}
