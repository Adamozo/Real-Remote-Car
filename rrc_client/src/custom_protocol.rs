use crate::car_state::{Gear, Pedal};

#[derive(Debug, Clone)]
pub enum ProtocolVersion {
    V1,
}

#[derive(Debug)]
pub enum ProtocolError {
    InvalidValue(String),
    // EncodingError(String),
}

// ------------------------

pub trait ProtocolEncoder {
    fn encode_gear(&self, gear: &Gear) -> Result<Vec<u8>, ProtocolError>;
    fn encode_pedal(&self, pedal: &Pedal) -> Result<Vec<u8>, ProtocolError>;
    fn encode_steering(&self, angle: i16) -> Result<Vec<u8>, ProtocolError>;
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
                    return Err(ProtocolError::InvalidValue("Gear number too high".into()));
                }
                Ok(format!("15+{}", n).as_bytes().to_vec())
            }
        }
    }

    fn encode_pedal(&self, pedal: &Pedal) -> Result<Vec<u8>, ProtocolError> {
        let coded_pedal = match pedal {
            Pedal::BREAK(value) => format!(
                "13{}{:03}",
                if *value >= 0 { "+" } else { "-" },
                value.abs()
            ),
            Pedal::GAS(value) => format!(
                "12{}{:03}",
                if *value >= 0 { "+" } else { "-" },
                value.abs()
            ),
            Pedal::CLUTCH(value) => format!(
                "14{}{:03}",
                if *value >= 0 { "+" } else { "-" },
                value.abs()
            ),
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
}
