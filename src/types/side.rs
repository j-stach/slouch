
use crate::error::BadElementError;

/// Market side (i.e., buyer/seller)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {

    /// Purchase shares
    Buy,

    /// Sell shares you own
    Sell,

    /// Sell shares you do not own, borrowing them to sell with the 
    /// intent to buy back later at a lower price. 
    /// Requires a locate be confirmed for borrowable shares.
    SellShort,

    /// Short sale order exempt from certain Regulation SHO restrictions, 
    /// such as the uptick rule or locate requirement.
    SellShortExempt,
}

impl Side {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use Side::*;
        match self {
            Buy             => b'B',
            Sell            => b'S',
            SellShort       => b'T',
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

