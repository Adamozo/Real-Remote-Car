use crate::config::MqttConfig;
use rumqttc::{Client, MqttOptions, Publish, QoS};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};

pub struct MqttConsumer {
    client: Client,
    config: MqttConfig,
    receiver: Receiver<Publish>,
}

impl MqttConsumer {
    pub fn new(config: MqttConfig) -> Result<Self, Box<dyn Error>> {
        let mut mqtt_options = MqttOptions::new(
            format!("{}-consumer", config.client_id),
            &config.host,
            config.port,
        );

        mqtt_options
            .set_clean_session(false)
            .set_keep_alive(Duration::from_secs(config.keep_alive_secs))
            .set_credentials(&config.username, &config.password);

        let (client, mut connection) = Client::new(mqtt_options, 10);

        let (sender, receiver) = mpsc::channel(100);

        thread::spawn(move || {
            for notification in connection.iter() {
                if let Ok(notification) = notification {
                    if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) =
                        notification
                    {
                        //println!("Received: topic: {}", publish.topic,);

                        if sender.blocking_send(publish).is_err() {
                            eprintln!("Receiver dropped, ending consumer thread");
                            break;
                        }
                    }
                }
            }
        });

        Ok(MqttConsumer {
            client,
            config,
            receiver,
        })
    }

    pub fn subscribe(&mut self, topic: &str) -> Result<(), Box<dyn Error>> {
        self.client.subscribe(topic, QoS::AtLeastOnce)?;
        Ok(())
    }

    pub async fn try_next(&mut self) -> Result<Option<Publish>, Box<dyn Error>> {
        match self.receiver.try_recv() {
            Ok(publish) => Ok(Some(publish)),
            Err(mpsc::error::TryRecvError::Empty) => Ok(None),
            Err(mpsc::error::TryRecvError::Disconnected) => {
                Err("MQTT consumer channel disconnected".into())
            }
        }
    }

    pub async fn next(&mut self) -> Result<Publish, Box<dyn Error>> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| "MQTT consumer channel disconnected".into())
    }
}
