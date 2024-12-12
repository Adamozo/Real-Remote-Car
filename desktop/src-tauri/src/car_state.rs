use crate::{
    controller::ControllerCommand,
    custom_protocol::{CustomProtocol, ProtocolError},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use ControllerCommand::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gear {
    REVERSE,
    NEUTRAL,
    FORWARD(u8),
}

impl Gear {
    const MAX_FORWARD_GEAR: u8 = 6;

    pub fn up(&self) -> Self {
        match self {
            Gear::REVERSE => Gear::NEUTRAL,
            Gear::NEUTRAL => Gear::FORWARD(1),
            Gear::FORWARD(n) if *n < Self::MAX_FORWARD_GEAR => Gear::FORWARD(n + 1),
            _ => self.clone(),
        }
    }

    pub fn down(&self) -> Self {
        match self {
            Gear::FORWARD(1) => Gear::NEUTRAL,
            Gear::FORWARD(n) => Gear::FORWARD(n - 1),
            Gear::NEUTRAL => Gear::REVERSE,
            Gear::REVERSE => Gear::REVERSE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pedal {
    GAS(u8),
    BREAK(u8),
    CLUTCH(u8),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IgnitionState {
    NEUTRAL,
    ON,
    OFF,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarState {
    pub current_gear: Gear,
    pub dirrect_gear: Gear,
    pub pbreak: Pedal,
    pub pclutch: Pedal,
    pub pgas: Pedal,
    pub stearing_wheel_angle: i16,
    pub ignition: IgnitionState,
    pub handbrake: u8,
}

impl CarState {
    pub fn new() -> Self {
        CarState {
            current_gear: Gear::NEUTRAL,
            dirrect_gear: Gear::NEUTRAL,
            pbreak: Pedal::BREAK(0),
            pclutch: Pedal::CLUTCH(0),
            pgas: Pedal::GAS(0),
            stearing_wheel_angle: 0,
            ignition: IgnitionState::NEUTRAL,
            handbrake: 0,
        }
    }

    pub fn _generate_random() -> Self {
        let mut rng = rand::thread_rng();

        let random_gear = match rng.gen_range(0..=6) {
            0 => Gear::REVERSE,
            1 => Gear::NEUTRAL,
            n => Gear::FORWARD(n as u8 - 1),
        };

        CarState {
            current_gear: random_gear.clone(),
            dirrect_gear: random_gear,
            pbreak: Pedal::BREAK(rng.gen_range(2..=4)),
            pclutch: Pedal::CLUTCH(rng.gen_range(2..=4)),
            pgas: Pedal::GAS(rng.gen_range(2..=4)),
            stearing_wheel_angle: rng.gen_range(2..=4),
            ignition: IgnitionState::NEUTRAL,
            handbrake: rng.gen_range(1..=100),
        }
    }

    pub fn execute_commad(&mut self, command: ControllerCommand) {
        // println!("{:?}", &command);

        match command {
            AutoGearDown => self.current_gear = self.current_gear.down(),
            AutoGearUp => self.current_gear = self.current_gear.up(),
            DirectGearDown => self.dirrect_gear = self.dirrect_gear.down(),
            DirectGearUp => self.dirrect_gear = self.dirrect_gear.up(),
            Starting(state) => match state {
                -1 => self.ignition = IgnitionState::OFF,
                0 => self.ignition = IgnitionState::NEUTRAL,
                1 => self.ignition = IgnitionState::ON,
                _ => unimplemented!(),
            },
            Cluch(value) => self.pclutch = Pedal::CLUTCH(value),
            Gas(value) => self.pgas = Pedal::GAS(value),
            Break(value) => self.pbreak = Pedal::BREAK(value),
            Handbrake(value) => self.handbrake = value,
            SubMenu1(on) => {
                if !on {
                    self.ignition = IgnitionState::NEUTRAL;
                }
            }
            SubMenu2(on) => {
                if on {
                    self.dirrect_gear = self.current_gear.clone()
                } else {
                    self.current_gear = self.dirrect_gear.clone()
                }
            }
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
        if self.ignition != previous.ignition {
            changes.push(protocol.encode_ignition(&self.ignition)?);
        }
        if self.handbrake != previous.handbrake {
            changes.push(protocol.encode_handbrake(self.handbrake)?);
        }

        Ok(changes)
    }
}
