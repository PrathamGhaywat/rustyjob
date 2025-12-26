use crate::storage::Storage;
use tokio::sync::mpsc;
use uuid::Uuid;

pub async fn start_scheduler(_storage: Storage, _tx: mpsc::Sender<Uuid>) -> anyhow::Result<()> {
    // TODO: implement scanning, scheduling, dependency checks, cron
    Ok(())
}