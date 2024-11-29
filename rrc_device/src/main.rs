mod config;
mod mqtt;
use crate::config::AppConfig;
use crate::mqtt::producer::MqttProducer;
use std::error::Error;
use std::time::Duration;
use tokio::time;

use tokio::{self};

pub async fn ping(config: AppConfig) -> Result<(), Box<dyn Error>> {
    let mut producer = MqttProducer::new(config.mqtt.clone())?;

    let ping_sender = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(4));

        loop {
            interval.tick().await;

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            if let Err(e) = producer.publish("car/ping", &timestamp) {
                eprintln!("Błąd wysyłania pinga: {}", e);
            }
        }
    });

    tokio::try_join!(ping_sender)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load("config.yaml")?;
    ping(config).await
}
