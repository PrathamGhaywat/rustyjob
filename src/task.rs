use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Pending,
    Scheduled,
    Ready,
    Running,
    Success,
    Failed,
    Blocked,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub kind: String, // "stdout" | "stderr" | "system"
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub max_retries: u32,
    pub attempts: u32,
    pub status: TaskStatus,
    pub last_run: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(name: impl Into<String>, command: impl Into<String>, max_retries: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            command: command.into(),
            max_retries,
            attempts: 0,
            status: TaskStatus::Pending,
            last_run: None,
        }
    }
}