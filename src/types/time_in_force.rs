
use crate::error::BadElementError;

///
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

    pub(crate) fn encode(&self) -> char {
        // TODO

        use Self::*;
        match self {
            Day => b'0', 
            IOC => b'3', 
            GTX => b'5', 
            GTT => b'6', 
            E => b'E', 
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data as char {
            b'0' => Day,
            b'3' => IOC,
            b'5' => GTX,
            b'6' => GTT,
            b'E' => E,
            _ => BadElementError::InvalidEnum(
                data as char, 
                "TimeInForce".to_string()
            )
        }
    }
}
