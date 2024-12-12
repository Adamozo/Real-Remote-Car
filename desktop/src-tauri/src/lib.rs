use rand::Rng;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
use tauri::Window;

#[derive(Serialize, Deserialize, Clone)]
struct CarState {
    gear: String,
    gas_position: u8,
    clutch_position: u8,
    brake_position: u8,
    speed: u16,
}

fn generate_random_state() -> CarState {
    let mut rng = rand::thread_rng();
    CarState {
        gear: match rng.gen_range(0..7) {
            0 => "R".to_string(),
            1 => "N".to_string(),
            n => n.to_string(),
        },
        gas_position: rng.gen_range(0..101),
        clutch_position: rng.gen_range(0..101),
        brake_position: rng.gen_range(0..101),
        speed: rng.gen_range(0..220),
    }
}

#[tauri::command]
async fn start_car_state_updates(window: Window) {
    std::thread::spawn(move || loop {
        let state = generate_random_state();
        window.emit("car-state-update", state).unwrap();
        thread::sleep(Duration::from_millis(10));
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_car_state_updates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
