use std::sync::mpsc::Sender;

use gilrs::EventType::{AxisChanged, ButtonChanged};
use gilrs::{Button, Event, GamepadId, Gilrs};

#[derive(Debug)]
pub enum ControllerCommand {
    AutoGearDown,
    AutoGearUp,
    DirectGearDown,
    DirectGearUp,
    Starting(i8),
    Cluch(u8),
    Gas(u8),
    Break(u8),
    Handbrake(u8),
    SubMenu1(bool),
    SubMenu2(bool),
}

#[derive(Debug)]
pub struct Controller {
    pad_id: GamepadId,
}

impl Controller {
    pub fn new() -> Result<Self, String> {
        let gilrs = Gilrs::new().unwrap();

        let pad = gilrs
            .gamepads()
            .find(|(_, pad)| pad.name() == "PS4 Controller");

        match pad {
            Some((pad_id, _)) => Ok(Controller { pad_id: pad_id }),
            None => Err("PS4 Controller not found".to_string()),
        }
    }

    pub fn run(&self, tx: Sender<ControllerCommand>) {
        let mut gilrs = Gilrs::new().unwrap();
        use ControllerCommand::*;

        loop {
            while let Some(Event { id, event, .. }) =
                gilrs.next_event().filter(|event| event.id == self.pad_id)
            {
                let gamepad = gilrs.gamepad(id);

                let msg = {
                    match event {
                        ButtonChanged(button, value, _) => match button {
                            Button::DPadDown if value == 1.0 => {
                                if gamepad.is_pressed(Button::RightTrigger) {
                                    Some(DirectGearDown)
                                } else {
                                    Some(AutoGearDown)
                                }
                            }
                            Button::DPadUp if value == 1.0 => {
                                if gamepad.is_pressed(Button::RightTrigger) {
                                    Some(DirectGearUp)
                                } else {
                                    Some(AutoGearUp)
                                }
                            }
                            Button::Start => {
                                let direction = (value as i8)
                                    * (if gamepad.is_pressed(Button::LeftTrigger) {
                                        -1
                                    } else {
                                        1
                                    });
                                Some(Starting(direction))
                            }
                            Button::LeftTrigger2 => {
                                let pedal_value = (value * 100.0) as u8;
                                if gamepad.is_pressed(Button::RightTrigger) {
                                    Some(Handbrake(pedal_value))
                                } else {
                                    Some(Cluch(pedal_value))
                                }
                            }
                            Button::RightTrigger2 => {
                                let pedal_value = (value * 100.0) as u8;

                                if gamepad.is_pressed(Button::LeftTrigger) {
                                    Some(Break(pedal_value))
                                } else {
                                    Some(Gas(pedal_value))
                                }
                            }
                            Button::LeftTrigger => Some(SubMenu1(value == 1.0)),
                            Button::RightTrigger => Some(SubMenu2(value == 1.0)),
                            _ => None, // println!("{:?} {} {}", button, value, code),
                        },
                        AxisChanged(_axis, _value, _) => None, // TODO stearing wheel location
                        _ => None,
                    }
                };

                if let Some(msg) = msg {
                    tx.send(msg).unwrap();
                }
            }
        }
    }
}
