use serde::{Deserialize, Serialize};
use tracing::{info, trace, trace_span};

use crate::{
    config::Config,
    maa,
    task::{StartUpParam, TaskRunningState, TaskStatus, TaskType},
    InstHandle,
};

#[derive(Default, Serialize, Deserialize)]
pub struct TaskQueue {
    queue: Vec<TaskStatus>,
}

impl TaskQueue {
    pub fn push(&mut self, task: TaskType) {
        self.queue.push(task.into());
    }

    /// Append a task to right after the running task
    pub fn append_next(&mut self, task: TaskType) {
        if let Some(index) = self
            .queue
            .iter()
            .position(|t| matches!(t.state, TaskRunningState::Pending))
        {
            self.queue.insert(index, task.into());
        } else {
            self.queue.push(task.into());
        }
    }

    /// Remove a task from the queue
    pub fn remove(&mut self, index: usize) {
        self.queue.remove(index);
    }

    /// Mark the running task as completed
    pub fn complete_running(&mut self) {
        if let Some(index) = self
            .queue
            .iter()
            .position(|t| matches!(t.state, TaskRunningState::Running))
        {
            self.queue[index].state = TaskRunningState::Completed;
        }
    }

    /// Mark the running task as completed and start the next task
    pub fn run_next(&mut self, handle: InstHandle, config: &Config) -> bool {
        let span = trace_span!("run_next");
        let _guard = span.enter();
        self.complete_running();
        trace!("Running next task");
        if let Some(index) = self
            .queue
            .iter()
            .position(|t| matches!(t.state, TaskRunningState::Pending))
        {
            self.queue[index].state = TaskRunningState::Running;
            let task = &self.queue[index];
            info!("Running task {:?}", task);
            match task.task_type {
                TaskType::StartUp => {
                    let start_up_config = config.start_up.clone();
                    let start_up_param: StartUpParam = start_up_config.into();
                    maa::post_task(handle, task.task_type, &start_up_param);
                }
            }
            true
        } else {
            info!("No more tasks to run");
            false
        }
    }
}
