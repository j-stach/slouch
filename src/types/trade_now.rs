
use crate::error::BadElementError;

/// Should.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeNow {

    Yes,

    No,

    /// Use Port default
    PortDefault
}

impl TradeNow {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use TradeNow::*;
        match data {
            b'Y' => Ok(Yes),
            b'N' => Ok(No),
            b' ' => Ok(PortDefault),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "TradeNow".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use TradeNow::*;
        match self {
            Yes => b'Y',
            No  => b'N',
            PortDefault => b' ',
        }
    }

}

impl Default for TradeNow {
    fn default() -> Self {
        TradeNow::PortDefault
    }
}
