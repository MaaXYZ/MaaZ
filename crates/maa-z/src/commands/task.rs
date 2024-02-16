use tauri::State;

use crate::{
    error::MaaResult,
    task::TaskType,
    TaskQueueState
};

macro_rules! task_command {
    ($name:ident,$upper_name:ident,$param:tt) => {
        #[tauri::command]
        pub async fn $name(
            task_queue: State<'_, TaskQueueState>,
            append_next: bool,
        ) -> MaaResult<()> {
            tracing::info!("Adding task {} to queue", stringify!($name));
            let mut queue = task_queue.lock().await;
            if append_next {
                queue.append_next(TaskType::$upper_name);
            } else {
                queue.push(TaskType::$upper_name);
            }
            Ok(())
        }
    };
}

task_command!(start_up, StartUp, StartUpParam);
