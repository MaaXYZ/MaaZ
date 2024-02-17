use tauri::State;
use tracing::info;

use crate::{
    task::{TaskStatus, TaskType}, ConfigHolderState, InstHandle, MaaResult, TaskQueueState
};

#[tauri::command]
pub async fn add_task_to_queue(
    task_queue: State<'_, TaskQueueState>,
    task: String,
    append_next: Option<bool>,
) -> MaaResult<()> {
    info!("Adding task {} to queue", task);
    let append_next = append_next.unwrap_or(false);
    let mut queue = task_queue.lock().await;
    let task_type = TaskType::try_from(task)?;
    if append_next {
        queue.append_next(task_type);
    } else {
        queue.push(task_type);
    }
    Ok(())
}

#[tauri::command]
pub async fn start_queue(task_queue: State<'_, TaskQueueState>, inst: State<'_, InstHandle>, config: State<'_,ConfigHolderState>) -> MaaResult<()> {
    tracing::info!("Starting task queue");
    let mut queue = task_queue.lock().await;
    let config = config.clone();
    let config = config.lock().await;
    queue.start(*inst, config.config());
    Ok(())
}

#[tauri::command]
pub async fn remove_from_queue(
    task_queue: State<'_, TaskQueueState>,
    index: usize,
) -> MaaResult<()> {
    tracing::info!("Removing task from queue");
    let mut queue = task_queue.lock().await;
    queue.remove(index);
    Ok(())
}

#[tauri::command]
pub async fn get_queue(task_queue: State<'_, TaskQueueState>) -> MaaResult<Vec<TaskStatus>> {
    let queue = task_queue.lock().await;
    Ok(queue.current_queue())
}
