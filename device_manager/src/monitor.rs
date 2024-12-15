use crate::config::AppConfig;
use crate::mqtt::consumer::MqttConsumer;
use crate::mqtt::producer::MqttProducer;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn ping_consumer(config: Arc<AppConfig>) {
    let mut consumer = MqttConsumer::new(config.mqtt.clone()).unwrap();
    consumer.subscribe(&config.monitor.ping_consumer).unwrap();

    loop {
        if let Some(msg) = consumer.try_next().unwrap() {
            let current_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let msg_timestamp = String::from_utf8_lossy(&msg.payload)
                .parse::<u64>()
                .unwrap();

            let diff_ms = current_timestamp - msg_timestamp;
            println!("Delay: {}ms", diff_ms);
        }
    }
}

pub fn ping_producer(config: Arc<AppConfig>) {
    let mut producer = MqttProducer::new(config.mqtt.clone()).unwrap();

    loop {
        std::thread::sleep(Duration::from_secs(config.monitor.producer_delay));

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if let Err(e) = producer.publish(&config.monitor.ping_producer, &timestamp) {
            eprintln!("Unable to send message: {}", e);
        }
    }
}
