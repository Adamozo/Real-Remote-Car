use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::AppConfig;
use crate::mqtt::consumer::MqttConsumer;

pub async fn ping(config: AppConfig) -> Result<(), Box<dyn Error>> {
    let mut consumer = MqttConsumer::new(config.mqtt).unwrap();
    consumer.subscribe("car/ping").unwrap();

    loop {
        if let Some(msg) = consumer.try_next().await.unwrap() {
            let current_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let msg_timestamp = String::from_utf8_lossy(&msg.payload)
                .parse::<u64>()
                .unwrap();

            let diff_ms = current_timestamp - msg_timestamp;
            println!("Różnica: {}ms", diff_ms);
        }
    }
}
