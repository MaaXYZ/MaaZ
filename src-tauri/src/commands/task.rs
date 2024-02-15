use tauri::State;

use crate::{
    error::MaaResult,
    maa,
    task::{StartUpParam, TaskType},
    ConfigHolderState, InstHandle,
};

macro_rules! task_command {
    ($name:ident,$upper_name:ident,$param:tt) => {
        #[tauri::command]
        pub async fn $name(
            handle: State<'_, InstHandle>,
            config_holder: State<'_, ConfigHolderState>,
        ) -> MaaResult<i64> {
            tracing::info!("Starting task {}", stringify!($name));
            let config_holder = config_holder.lock().await;
            let config = config_holder.config();
            let param: $param = config.$name.clone().into();
            Ok(maa::post_task(&handle, &TaskType::$upper_name, &param))
        }
    };
}

task_command!(start_up, StartUp, StartUpParam);
