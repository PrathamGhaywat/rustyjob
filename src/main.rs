use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod logging;
mod task;
mod storage;
mod scheduler;
mod executor;

#[derive(Parser)]
#[command(name = "rustyjob")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
    #[arg(long, default_value = "rustyjob.db")]
    db: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        command: String,
        #[arg(long, default_value_t = 0)]
        retries: u32,
    },
    Run {
        id: String,
    },
    Status {
        id: String,
    },
    Logs {
        id: String,
        #[arg(long, default_value_t = 100)]
        tail: usize,
    },
    Daemon {
        #[arg(long)]
        start: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init_tracing()?;
    let cli = Cli::parse();
    let storage = storage::Storage::open(cli.db)?;
    match cli.cmd {
        Commands::Add { name, command, retries } => {
            let t = task::Task::new(name, command, retries);
            storage.insert_task(&t)?;
            println!("{}", t.id);
        }
        Commands::Run { id } => {
            // TODO: schedule immediate run via scheduler/executor
            println!("run {}", id);
        }
        Commands::Status { id } => {
            if let Some(t) = storage.get_task(&id.parse().unwrap())? {
                println!("{:#?}", t);
            }
        }
        Commands::Logs { id, tail } => {
            let uuid = id.parse().unwrap();
            let entries = storage.get_logs(uuid, tail)?;
            for e in entries { println!("[{}] {}", e.timestamp, e.content); }
        }
        Commands::Daemon { start: _ } => {
            // TODO: start scheduler loop
            println!("daemon TODO");
        }
    }
    Ok(())
}