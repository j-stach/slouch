
use crate::error::BadElementError;

///
pub enum Side {
    Buy,
    Sell,
    SellShort,
    SellShortExempt,
}

impl Side {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Buy => b'B',
            Sell => b'S',
            SellShort => b'T',
            SellShortExempt => b'E',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'B' => Ok(Buy),
            b'S' => Ok(Sell),
            b'T' => Ok(SellShort),
            b'E' => Ok(SellShortExempt),
            _ => BadElementError::InvalidEnum(
                data as char, 
                "Side".to_string()
            )
        }
    }
}

