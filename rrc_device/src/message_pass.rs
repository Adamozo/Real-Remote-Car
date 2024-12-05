use crate::config::AppConfig;
use crate::mqtt::consumer::MqttConsumer;
use std::time::Duration;
use tokio::time;

pub async fn message_consumer(config: AppConfig) {
    let mut consumer =
        MqttConsumer::new(config.mqtt.clone(), config.mqtt.task_topic.clone()).unwrap();
    consumer.subscribe().unwrap();

    loop {
        match consumer.try_next().await {
            Ok(m) => {
                if let Some(msg) = m {
                    let message = String::from_utf8_lossy(&msg.payload);

                    println!("MESSAGE: {}", message);
                }
            }
            e => println!("{:?}", e),
        }
    }
}
