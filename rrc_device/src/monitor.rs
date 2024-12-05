use crate::config::AppConfig;
use crate::mqtt::consumer::MqttConsumer;
use crate::mqtt::producer::MqttProducer;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time;

pub fn ping_consumer(config: Arc<AppConfig>) {
    let mut consumer =
        MqttConsumer::new(config.mqtt.clone(), config.monitor.ping_consumer.clone()).unwrap();
    consumer.subscribe().unwrap();

    loop {
        match consumer.try_next() {
            Ok(m) => {
                if let Some(msg) = m {
                    let current_timestamp = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    let msg_t = String::from_utf8_lossy(&msg.payload).parse::<u64>();

                    match msg_t {
                        Ok(msg_timestamp) => {
                            let diff_ms = current_timestamp - msg_timestamp;
                            println!("Delay: {}ms", diff_ms);
                        }
                        e => println!("Error parsing timestamp from topic {}: {:?}", msg.topic, e),
                    }
                }
            }
            e => println!("Error receiving message: {:?}", e),
        }
    }
}

pub fn ping_producer(config: Arc<AppConfig>) {
    let mut producer = MqttProducer::new(config.mqtt.clone()).unwrap();
    let mut interval = time::interval(Duration::from_secs(config.monitor.producer_delay));

    loop {
        interval.tick();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if let Err(e) = producer.publish(&config.monitor.ping_producer, &timestamp) {
            eprintln!("Unable to send message: {}", e);
        }
    }
}
