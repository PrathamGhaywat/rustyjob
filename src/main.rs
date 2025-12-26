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
        name: String,
    },
    Status {
        name: String,
    },
    Logs {
        name: String,
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
        Commands::Run { name } => {
            if let Some(t) = storage.get_task_by_name(&name)? {
                // TODO: schedule immediate run via scheduler/executor
                println!("would run task {} ({})", t.name, t.id);
            } else {
                eprintln!("task not found: {}", name);
            }
        }
        Commands::Status { name } => {
            if let Some(t) = storage.get_task_by_name(&name)? {
                println!("{:#?}", t);
            } else {
                eprintln!("task not found: {}", name);
            }
        }
        Commands::Logs { name, tail } => {
            if let Some(t) = storage.get_task_by_name(&name)? {
                let entries = storage.get_logs(t.id, tail)?;
                for e in entries { println!("[{}] {}: {}", e.timestamp, e.kind, e.content); }
            } else {
                eprintln!("task not found: {}", name);
            }
        }
        Commands::Daemon { start: _ } => {
            // TODO: start scheduler loop
            println!("daemon TODO");
        }
    }
    Ok(())
}