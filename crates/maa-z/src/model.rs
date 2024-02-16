use serde::{Deserialize, Serialize};

use crate::task::TaskType;

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub adb_config: String,
    pub adb_serial: String,
    pub controller_type: i32,
    pub adb_path: String,
}

#[derive(Serialize, Deserialize)]
pub enum TaskRunningState {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize)]
pub struct TaskStatus {
    pub id: Option<i64>,
    pub task_type: TaskType,
    pub state: TaskRunningState
}

impl From<TaskType> for TaskStatus {
    fn from(task_type: TaskType) -> Self {
        Self {
            id: None,
            task_type,
            state: TaskRunningState::Pending,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct TaskQueue {
    queue: Vec<TaskStatus>,
}

impl TaskQueue {
    pub fn push(&mut self, task: TaskType) {
        self.queue.push(task.into());
    }

    /// Get the first task that is pending
    pub fn find_next(&self) -> Option<&TaskStatus> {
        self.queue.iter().find(|t| matches!(t.state, TaskRunningState::Pending))
    }

    pub fn append_next(&mut self, task: TaskType) {
        if let Some(index) = self.queue.iter().position(|t| matches!(t.state, TaskRunningState::Pending)) {
            self.queue.insert(index, task.into());
        } else {
            self.queue.push(task.into());
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.queue.remove(index);
    }
}