
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventCode {

    /// Indicates that exchange is open and is ready to accept orders.
    StartOfDay,

    /// Indicates that the exchange is closed and will no longer accept new orders. 
    /// Note that it is still possible to receive breaks and cancels after this event.
    EndOfDay
}

impl EventCode {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use EventCode::*;
        match data {
            b'S' => Ok(StartOfDay),
            b'E' => Ok(EndOfDay),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "EventCode".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use EventCode::*;
        match self {
            StartOfDay  => b'S',
            EndOfDay    => b'E',
        }
    }

}
