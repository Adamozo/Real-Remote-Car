use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

use crate::car_state::CarState;
use crate::config::AppConfig;
use crate::mqtt::consumer::MqttConsumer;
use crate::mqtt::producer::MqttProducer;

pub async fn run_loop(config: AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(CarState::new()));
    let state_for_update = state.clone();

    let state_updater = tokio::spawn(async move {
        loop {
            {
                let mut state = state_for_update.lock().unwrap();
                *state = CarState::generate_random();
            }
            time::sleep(Duration::from_millis(config.update_interval_ms)).await;
        }
    });

    let state_printer = tokio::spawn(async move {
        let mut previous_state = CarState::new();
        loop {
            {
                let current_state = state.lock().unwrap();
                let changes = current_state.get_changes(&previous_state);
                if !changes.is_empty() {
                    println!("{}", changes.join(", "));
                }
                previous_state = current_state.clone();
            }
            time::sleep(Duration::from_millis(config.update_interval_ms)).await;
        }
    });

    tokio::try_join!(state_updater, state_printer)?;
    Ok(())
}
