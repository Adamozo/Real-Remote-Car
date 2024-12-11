mod car_state;
mod config;
mod controller;
mod custom_protocol;
mod monitor;
mod mqtt;
mod runner;

use crate::config::AppConfig;
use crate::runner::run_loop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load("config.yaml")?;

    run_loop(config)?;

    Ok(())
}
