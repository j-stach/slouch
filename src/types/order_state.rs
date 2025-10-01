
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderState {

    ///
    Live,

    /// Means that the order was accepted and automatically canceled.
    Dead
}

impl OrderState {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use OrderState::*;
        match data {
            b'L' => Ok(Live),
            b'D' => Ok(Dead),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "OrderState".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use OrderState::*;
        match self {
            Live => b'L',
            Dead => b'D',
        }
    }

}
