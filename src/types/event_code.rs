
use crate::error::BadElementError;

/// Identifier for a system-wide signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCode {

    /// Indicates that exchange is open and is ready to accept orders.
    StartOfDay,

    /// Exchange is closed and will no longer accept new orders. 
    /// NOTE: Still possible to receive breaks and cancels after this event.
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

    #[allow(dead_code)] // Future use
    pub(crate) fn encode(&self) -> u8 {
        
        use EventCode::*;
        match self {
            StartOfDay  => b'S',
            EndOfDay    => b'E',
        }
    }

}
