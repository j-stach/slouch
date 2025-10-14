
use crate::error::BadElementError;

/// Visibility options for an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {

    /// The order is fully displayed in the public order book, 
    /// showing the full quantity at the specified price.
    Visible,

    /// The order is not visible in the public book but can still match 
    /// against incoming orders.
    Hidden,

    /// The order is visible but attributed to a specific 
    /// Market Participant Identifier (MPID), rather than being anonymous.
    Attributable,
}

impl Display {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use Display::*;
        match self {
            Visible => b'Y',
            Hidden => b'N',
            Attributable => b'A',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Display::*;
        match data {
            b'Y' => Ok(Visible),
            b'N' => Ok(Hidden),
            b'A' => Ok(Attributable),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "Display".to_string()
            ))
        }
    }
}

