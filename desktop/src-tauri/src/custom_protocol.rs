use crate::car_state::{Gear, IgnitionState, Pedal};

#[derive(Debug, Clone)]
pub enum ProtocolVersion {
    V1,
}

#[derive(Debug)]
pub enum ProtocolError {
    InvalidValue,
    // EncodingError(String),
}

// ------------------------

pub trait ProtocolEncoder {
    fn encode_gear(&self, gear: &Gear) -> Result<Vec<u8>, ProtocolError>;
    fn encode_pedal(&self, pedal: &Pedal) -> Result<Vec<u8>, ProtocolError>;
    fn encode_steering(&self, angle: i16) -> Result<Vec<u8>, ProtocolError>;
    fn encode_ignition(&self, state: &IgnitionState) -> Result<Vec<u8>, ProtocolError>;
    fn encode_handbrake(&self, value: u8) -> Result<Vec<u8>, ProtocolError>;
}

// ------------------------

pub struct CustomProtocolV1;

impl ProtocolEncoder for CustomProtocolV1 {
    fn encode_gear(&self, gear: &Gear) -> Result<Vec<u8>, ProtocolError> {
        match gear {
            Gear::REVERSE => Ok("15-1".as_bytes().to_vec()),
            Gear::NEUTRAL => Ok("15+0".as_bytes().to_vec()),
            Gear::FORWARD(n) => {
                if *n > 6 {
                    return Err(ProtocolError::InvalidValue);
                }
                Ok(format!("15+{}", n).as_bytes().to_vec())
            }
        }
    }

    fn encode_pedal(&self, pedal: &Pedal) -> Result<Vec<u8>, ProtocolError> {
        let coded_pedal = match pedal {
            Pedal::BREAK(value) => format!("13+{:03}", value),
            Pedal::GAS(value) => format!("12+{:03}", value),
            Pedal::CLUTCH(value) => {
                format!("14+{:03}", value)
            }
        };

        Ok(coded_pedal.as_bytes().to_vec())
    }

    fn encode_steering(&self, angle: i16) -> Result<Vec<u8>, ProtocolError> {
        Ok(
            format!("11{}{:04}", if angle >= 0 { "+" } else { "-" }, angle.abs())
                .as_bytes()
                .to_vec(),
        )
    }

    fn encode_ignition(&self, state: &IgnitionState) -> Result<Vec<u8>, ProtocolError> {
        match state {
            IgnitionState::OFF => Ok("10-1".as_bytes().to_vec()),
            IgnitionState::NEUTRAL => Ok("10+0".as_bytes().to_vec()),
            IgnitionState::ON => Ok("10+1".as_bytes().to_vec()),
        }
    }

    fn encode_handbrake(&self, value: u8) -> Result<Vec<u8>, ProtocolError> {
        Ok(format!("16+{:04}", value).as_bytes().to_vec())
    }
}

// ------------------------

pub struct CustomProtocol {
    encoder: Box<dyn ProtocolEncoder>,
}

impl CustomProtocol {
    pub fn new(version: ProtocolVersion) -> Result<Self, ProtocolError> {
        let encoder: Box<dyn ProtocolEncoder> = match version {
            ProtocolVersion::V1 => Box::new(CustomProtocolV1),
        };

        Ok(CustomProtocol { encoder })
    }

    pub fn encode_gear(&self, gear: &Gear) -> Result<Vec<u8>, ProtocolError> {
        self.encoder.encode_gear(&gear)
    }

    pub fn encode_pedal(&self, pedal: &Pedal) -> Result<Vec<u8>, ProtocolError> {
        self.encoder.encode_pedal(pedal)
    }

    pub fn encode_steering(&self, angle: i16) -> Result<Vec<u8>, ProtocolError> {
        self.encoder.encode_steering(angle)
    }

    pub fn encode_handbrake(&self, value: u8) -> Result<Vec<u8>, ProtocolError> {
        self.encoder.encode_handbrake(value)
    }

    pub fn encode_ignition(&self, state: &IgnitionState) -> Result<Vec<u8>, ProtocolError> {
        self.encoder.encode_ignition(&state)
    }
}
