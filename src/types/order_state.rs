
use crate::error::BadElementError;

///
pub enum OrderState {

    ///
    Live,

    /// Means that the order was accepted and automatically canceled.
    Dead
}

impl EventCode {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'L' => Ok(Live),
            b'D' => Ok(Dead),

            _ => BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "OrderState".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Live => b'L',
            Dead => b'D',
        }
    }

}
