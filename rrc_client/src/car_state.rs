use crate::custom_protocol::{CustomProtocol, ProtocolError};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Gear {
    REVERSE,
    NEUTRAL,
    FORWARD(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pedal {
    GAS(i8),
    BREAK(i8),
    CLUTCH(i8),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CarState {
    pub current_gear: Gear,
    pub pbreak: Pedal,
    pub pclutch: Pedal,
    pub pgas: Pedal,
    pub stearing_wheel_angle: i16,
}

impl CarState {
    pub fn new() -> Self {
        CarState {
            current_gear: Gear::NEUTRAL,
            pbreak: Pedal::BREAK(0),
            pclutch: Pedal::CLUTCH(0),
            pgas: Pedal::GAS(0),
            stearing_wheel_angle: 0,
        }
    }

    pub fn generate_random() -> Self {
        let mut rng = rand::thread_rng();

        let random_gear = match rng.gen_range(0..=6) {
            0 => Gear::REVERSE,
            1 => Gear::NEUTRAL,
            n => Gear::FORWARD(n as u8 - 1),
        };

        CarState {
            current_gear: random_gear,
            pbreak: Pedal::BREAK(rng.gen_range(-2..=2)),
            pclutch: Pedal::CLUTCH(rng.gen_range(-2..=2)),
            pgas: Pedal::GAS(rng.gen_range(-2..=2)),
            stearing_wheel_angle: rng.gen_range(-2..=2),
        }
    }

    pub fn get_changes(
        &self,
        previous: &Self,
        protocol: &CustomProtocol,
    ) -> Result<Vec<Vec<u8>>, ProtocolError> {
        let mut changes = Vec::new();

        if self.current_gear != previous.current_gear {
            changes.push(protocol.encode_gear(&self.current_gear)?);
        }
        if self.pbreak != previous.pbreak {
            changes.push(protocol.encode_pedal(&self.pbreak)?);
        }
        if self.pclutch != previous.pclutch {
            changes.push(protocol.encode_pedal(&self.pclutch)?);
        }
        if self.pgas != previous.pgas {
            changes.push(protocol.encode_pedal(&self.pgas)?);
        }
        if self.stearing_wheel_angle != previous.stearing_wheel_angle {
            changes.push(protocol.encode_steering(self.stearing_wheel_angle)?);
        }

        Ok(changes)
    }
}
