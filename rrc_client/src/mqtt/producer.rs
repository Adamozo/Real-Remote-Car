use crate::config::MqttConfig;
use rumqttc::{Client, MqttOptions, QoS};
use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct MqttProducer {
    client: Client,
}

impl MqttProducer {
    pub fn new(config: MqttConfig) -> Result<Self, Box<dyn Error>> {
        let mut mqtt_options = MqttOptions::new(&config.client_id, &config.host, config.port);

        mqtt_options
            .set_clean_session(false)
            .set_keep_alive(Duration::from_secs(config.keep_alive_secs))
            .set_credentials(&config.username, &config.password);

        let (client, mut connection) = Client::new(mqtt_options, 10);

        thread::spawn(move || {
            for _notification in connection.iter() {
                //println!("Producer Event: {:?}", notification);
            }
        });

        Ok(MqttProducer { client })
    }

    pub fn publish<T: Serialize>(
        &mut self,
        topic: &str,
        payload: &T,
    ) -> Result<(), Box<dyn Error>> {
        let payload = serde_json::to_string(payload)?;
        //println!("Publishing to {}: {}", topic, payload);
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload.as_bytes())?;
        Ok(())
    }
}
