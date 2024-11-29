use crate::mqtt::*;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
enum Gear {
    REVERSE,
    NEUTRAL,
    FORWARD(u8),
}

#[derive(Debug, Clone, PartialEq)]
struct Pedal {
    value: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CarState {
    current_gear: Gear,
    pbreak: Pedal,
    pclutch: Pedal,
    pgas: Pedal,
    stearing_wheel_angle: i16,
}

impl CarState {
    pub fn new() -> Self {
        CarState {
            current_gear: Gear::NEUTRAL,
            pbreak: Pedal { value: 0 },
            pclutch: Pedal { value: 0 },
            pgas: Pedal { value: 0 },
            stearing_wheel_angle: 0,
        }
    }

    pub fn generate_random() -> Self {
        let mut rng = rand::thread_rng();

        // let random_gear = match rng.gen_range(0..=6) {
        //     0 => Gear::REVERSE,
        //     1 => Gear::NEUTRAL,
        //     n => Gear::FORWARD(n as u8 - 1),
        // };

        CarState {
            current_gear: Gear::NEUTRAL,
            pbreak: Pedal {
                value: rng.gen_range(-2..=2),
            },
            pclutch: Pedal {
                value: rng.gen_range(-2..=2),
            },
            pgas: Pedal {
                value: rng.gen_range(-2..=2),
            },
            stearing_wheel_angle: rng.gen_range(-2..=2),
        }
    }

    pub fn get_changes(&self, previous: &Self) -> Vec<String> {
        let mut changes = Vec::new();

        if self.current_gear != previous.current_gear {
            changes.push(format!(
                "gear: {:?} -> {:?}",
                previous.current_gear, self.current_gear
            ));
        }
        if self.pbreak != previous.pbreak {
            changes.push(format!(
                "break: {} -> {}",
                previous.pbreak.value, self.pbreak.value
            ));
        }
        if self.pclutch != previous.pclutch {
            changes.push(format!(
                "clutch: {} -> {}",
                previous.pclutch.value, self.pclutch.value
            ));
        }
        if self.pgas != previous.pgas {
            changes.push(format!(
                "gas: {} -> {}",
                previous.pgas.value, self.pgas.value
            ));
        }
        if self.stearing_wheel_angle != previous.stearing_wheel_angle {
            changes.push(format!(
                "wheel: {} -> {}",
                previous.stearing_wheel_angle, self.stearing_wheel_angle
            ));
        }

        changes
    }
}
