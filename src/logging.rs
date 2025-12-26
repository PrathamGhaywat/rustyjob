use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*; // brings `with` into scope
use anyhow::Result;

pub fn init_tracing() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .try_init()
        .ok();
    Ok(())
}