mod car_state;
mod config;
mod monitor;
mod mqtt;
mod runner;

use crate::config::AppConfig;
use crate::monitor::ping;
use crate::runner::run_loop;
use tokio::{self};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load("config.yaml")?;

    ping(config).await?;
    //run_loop(config).await?;
    Ok(())
}
