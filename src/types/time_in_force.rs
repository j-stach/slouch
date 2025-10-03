
use crate::error::BadElementError;

/// During which time span will the order be active?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    /// Market hours
    Day,
    /// IOC 
    IOC,
    /// Extended hours
    GTX,
    /// Specify ExpireTime in OptionalAppendage
    GTT,
    /// After hours
    E
}

impl TimeInForce {

    pub(crate) fn encode(&self) -> u8 {

        use TimeInForce::*;
        match self {
            Day => b'0', 
            IOC => b'3', 
            GTX => b'5', 
            GTT => b'6', 
            E   => b'E', 
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use TimeInForce::*;
        match data {
            b'0' => Ok(Day),
            b'3' => Ok(IOC),
            b'5' => Ok(GTX),
            b'6' => Ok(GTT),
            b'E' => Ok(E),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "TimeInForce".to_string()
            ))
        }
    }
}
