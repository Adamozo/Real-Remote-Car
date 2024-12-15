mod config;
mod message_pass;
mod monitor;
mod mqtt;

use crate::config::AppConfig;
use crate::message_pass::message_consumer;
use crate::monitor::{ping_consumer, ping_producer};
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(AppConfig::load("config.yaml")?);
    let mut handles = vec![];

    handles.push(thread::spawn({
        let config = Arc::clone(&config);
        move || {
            if !config.monitor.disable_ping {
                ping_producer(config)
            }
        }
    }));

    handles.push(thread::spawn({
        let config = Arc::clone(&config);
        move || {
            if !config.monitor.disable_ping {
                ping_consumer(config)
            }
        }
    }));

    handles.push(thread::spawn({
        let config = Arc::clone(&config);
        move || message_consumer(config)
    }));

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
