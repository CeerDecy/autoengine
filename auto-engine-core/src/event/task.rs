use serde::Serialize;

pub const TASK_EVENT: &str = "task";

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Running,
    Paused,
    Finished,
    Cancelled,
}

#[derive(Serialize, Clone)]
pub struct TaskEventPayload {
    pub status: TaskStatus,
}
