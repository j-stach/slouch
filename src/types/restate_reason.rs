
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestateReason {

    /// Refresh of display (on an order with reserves)
    Refresh,

    /// Update of displayed price
    Update
}

impl RestateReason {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use RestateReason::*;
        match data {
            b'R' => Ok(Refresh),
            b'P' => Ok(Update),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "OrderState".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use RestateReason::*;
        match self {
            Refresh => b'R',
            Update  => b'P',
        }
    }

}
