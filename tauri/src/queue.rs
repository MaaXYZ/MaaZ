use serde::{Deserialize, Serialize};
use tracing::{error, info, trace, trace_span};

use crate::{
    config::Config,
    maa,
    task::{AwardParam, StartUpParam, TaskRunningState, TaskStatus, TaskType},
    InstHandle,
};

#[derive(Default, Serialize, Deserialize)]
pub struct TaskQueue {
    queue: Vec<TaskStatus>,
}

#[derive(Debug)]
pub enum QueueStartStatus {
    Started,
    AlreadyRunning,
    NoPendingTasks,
}

impl TaskQueue {
    pub fn current_queue(&self) -> Vec<TaskStatus> {
        self.queue.clone()
    }

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
            let task = &mut self.queue[index];
            info!("Running task {:?}", task);
            let id = match task.task_type {
                TaskType::StartUp => {
                    let start_up_config = config.start_up.clone();
                    let start_up_param: StartUpParam = start_up_config.into();
                    maa::post_task(handle, task.task_type, &start_up_param)
                },
                TaskType::Award => {
                    let award_config = config.award.clone();
                    let award_param: AwardParam = award_config.into();
                    maa::post_task(handle, task.task_type, &award_param)
                },
            };
            task.id = Some(id);
            true
        } else {
            info!("No more tasks to run");
            false
        }
    }

    pub fn idle(&self) -> bool {
        !self
            .queue
            .iter()
            .any(|t| matches!(t.state, TaskRunningState::Running))
    }

    pub fn start(&mut self, handle: InstHandle, config: &Config) -> QueueStartStatus {
        if !self.idle() {
            info!("Task queue is already running");
            return QueueStartStatus::AlreadyRunning;
        }

        let has_pending = self
            .queue
            .iter()
            .any(|t| matches!(t.state, TaskRunningState::Pending));
        if !has_pending {
            info!("No pending tasks to run");
            return QueueStartStatus::NoPendingTasks;
        }

        self.run_next(handle, config);
        QueueStartStatus::Started
    }

    /// This sends a stop signal to fw and mark the running task as Pending
    pub fn stop(&mut self, handle: InstHandle) {
        let stop_ret = maa::stop_task(handle);
        if stop_ret.is_err() {
            error!("Error while stopping task");
        }
        if let Some(index) = self
            .queue
            .iter()
            .position(|t| matches!(t.state, TaskRunningState::Running))
        {
            self.queue[index].state = TaskRunningState::Pending;
        }
    }
}
