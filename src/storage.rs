use crate::task::{Task, LogEntry};
use anyhow::Result;
use std::path::Path;
use uuid::Uuid;

pub struct Storage {
    db: sled::Db,
    tasks: sled::Tree,
    logs: sled::Tree,
    names: sled::Tree,
}

impl Storage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)?;
        let tasks = db.open_tree("tasks")?;
        let logs = db.open_tree("logs")?;
        let names = db.open_tree("names")?;
        Ok(Self { db, tasks, logs, names })
    }

    pub fn insert_task(&self, task: &Task) -> Result<()> {
        let key = task.id.as_bytes();
        let val = bincode::serialize(task)?;
        self.tasks.insert(key, val)?;
        self.names.insert(task.name.as_bytes(), task.id.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get_task(&self, id: &Uuid) -> Result<Option<Task>> {
        if let Some(v) = self.tasks.get(id.as_bytes())? {
            let t: Task = bincode::deserialize(&v)?;
            Ok(Some(t))
        } else {
            Ok(None)
        }
    }

    pub fn get_task_by_name(&self, name: &str) -> Result<Option<Task>> {
        if let Some(id_bytes) = self.names.get(name.as_bytes())? {
            // id_bytes is IVec containing 16 bytes of UUID
            if let Ok(uuid) = Uuid::from_slice(&id_bytes) {
                return self.get_task(&uuid);
            }
        }
        for item in self.tasks.iter() {
            let (_k, v) = item?;
            let t: Task = bincode::deserialize(&v)?;
            if t.name == name {
                return Ok(Some(t));
            }
        }
        Ok(None)
    }

    pub fn append_log(&self, task_id: Uuid, entry: &LogEntry) -> Result<()> {
        let nanos = entry.timestamp.timestamp_nanos_opt().unwrap_or(0);
        let key = format!("{}:{}", task_id, nanos);
        let val = bincode::serialize(entry)?;
        self.logs.insert(key.as_bytes(), val)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get_logs(&self, task_id: Uuid, tail: usize) -> Result<Vec<LogEntry>> {
        let prefix = format!("{}:", task_id);
        let mut out = Vec::new();
        for item in self.logs.scan_prefix(prefix.as_bytes()) {
            let (_k, v) = item?;
            let e: LogEntry = bincode::deserialize(&v)?;
            out.push(e);
        }
        if out.len() > tail {
            out.reverse();
            out.truncate(tail);
            out.reverse();
        }
        Ok(out)
    }
}