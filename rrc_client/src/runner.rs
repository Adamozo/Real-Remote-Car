use crate::car_state::CarState;
use crate::config::AppConfig;
use crate::controller::Controller;
use crate::custom_protocol::{CustomProtocol, ProtocolVersion};
use crate::monitor::{ping_consumer, ping_producer};
use crate::mqtt::producer::MqttProducer;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

pub fn run_loop(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    let config = Arc::new(config);
    let mut children = vec![];
    // ---------------------------------------

    children.push(thread::spawn({
        let controller = Controller::new()?;
        let tx = tx.clone();
        move || {
            controller.run(tx);
        }
    }));

    // ---------------------------------------

    children.push(thread::spawn({
        let config = Arc::clone(&config);
        move || {
            if !config.monitor.disable_ping {
                ping_producer(config)
            }
        }
    }));

    // ---------------------------------------

    children.push(thread::spawn({
        let config = Arc::clone(&config);
        move || {
            if !config.monitor.disable_ping {
                ping_consumer(config)
            }
        }
    }));

    // ---------------------------------------

    children.push(thread::spawn({
        let config = Arc::clone(&config);
        let rx = rx;
        move || {
            let mut previous_state = CarState::new();
            let mut current_state = CarState::new();
            let mut producer = MqttProducer::new(config.mqtt.clone()).unwrap();
            let protocol = CustomProtocol::new(ProtocolVersion::V1).unwrap();
            let mut last_update = std::time::Instant::now();

            loop {
                let now = std::time::Instant::now();
                if now.duration_since(last_update).as_millis() >= config.update_interval_ms as u128
                {
                    last_update = now;
                    let changes = current_state
                        .get_changes(&previous_state, &protocol)
                        .unwrap();

                    if !changes.is_empty() {
                        let changes_str: Vec<String> = changes
                            .iter()
                            .map(|msg| String::from_utf8_lossy(msg).to_string())
                            .collect();
                        let message = changes_str.join(", ");
                        println!("{}", &message);
                        // uncomment to sned protocol data to mqtt
                        //producer.publish("sample", &message).unwrap();

                        if let Err(e) = producer.publish("tasks", &message) {
                            eprintln!("Unable to send message: {}", e);
                        }
                    }

                    previous_state = current_state.clone();
                }

                if let Ok(command) = rx.recv() {
                    current_state.execute_commad(command);
                }
            }
        }
    }));

    // ---------------------------------------

    for child in children {
        child.join().map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Thread panicked: {:?}", e),
            ))
        })?;
    }

    Ok(())
}
