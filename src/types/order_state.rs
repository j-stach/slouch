
use crate::error::BadElementError;

/// Should be a boolean, but in the interest of maintaining consistency with
/// the wording in the spec, we use an enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {

    /// Order is accepted and tracked in the system.
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

    #[allow(dead_code)] // Future use
    pub(crate) fn encode(&self) -> u8 {
        
        use OrderState::*;
        match self {
            Live => b'L',
            Dead => b'D',
        }
    }

}
