use crate::car_state::CarState;
use crate::config::AppConfig;
use crate::custom_protocol::{CustomProtocol, ProtocolVersion};
use crate::monitor::{ping_consumer, ping_producer};
use crate::mqtt::producer::MqttProducer;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn run_loop(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(CarState::new()));
    let state_for_update = Arc::clone(&state);
    let config = Arc::new(config);
    let mut children = vec![];

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
        move || loop {
            {
                let mut state = state_for_update.lock().unwrap();
                *state = CarState::generate_random();
            }
            thread::sleep(Duration::from_millis(config.update_interval_ms));
        }
    }));

    // ---------------------------------------

    children.push(thread::spawn({
        let config = Arc::clone(&config);
        move || {
            let mut previous_state = CarState::new();
            let mut producer = MqttProducer::new(config.mqtt.clone()).unwrap();
            let protocol = CustomProtocol::new(ProtocolVersion::V1).unwrap();

            loop {
                {
                    let current_state = state.lock().unwrap();
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
                        if let Err(e) = producer.publish("tasks", &message) {
                            eprintln!("Unable to send message: {}", e);
                        }
                    }
                    previous_state = current_state.clone();
                }
                std::thread::sleep(Duration::from_millis(config.update_interval_ms));
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
