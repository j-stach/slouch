
use crate::error::BadElementError;

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
    SellShort,
    SellShortExempt,
}

impl Side {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use Side::*;
        match self {
            Buy => b'B',
            Sell => b'S',
            SellShort => b'T',
            SellShortExempt => b'E',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Side::*;
        match data {
            b'B' => Ok(Buy),
            b'S' => Ok(Sell),
            b'T' => Ok(SellShort),
            b'E' => Ok(SellShortExempt),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "Side".to_string()
            ))
        }
    }
}

