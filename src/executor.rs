use anyhow::Result;
use crate::storage::Storage;
use crate::task::Task;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run_task(storage: &Storage, task: Task) -> Result<()> {
    // TODO: windows (cmd /C) vs raw exec
    let mut child = Command::new("cmd")
        .args(["/C", &task.command])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let mut reader = BufReader::new(stdout).lines();
        while let Some(line) = reader.next_line().await? {
            let entry = crate::task::LogEntry {
                timestamp: chrono::Utc::now(),
                kind: "stdout".into(),
                content: line,
            };
            storage.append_log(task.id, &entry)?;
        }
    }

    let status = child.wait().await?;
    // TODO: update task status, attempts, retries in storage
    if status.success() {
    } else {
    }
    Ok(())
}