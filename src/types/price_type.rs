
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriceType {

    ///
    Limit,

    ///
    MarketPeg,

    ///
    MidpointPeg,

    ///
    PrimaryPeg,

    ///
    /// Not used by `OptionValue::DiscretionPriceType`
    MarketMakerPeg,

    ///
    /// Not used by `OptionValue::DiscretionPriceType`
    // TODO: Clarify this with devs 
    Midpoint
}

impl PriceType {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'L' => Ok(Limit),
            b'P' => Ok(MarketPeg),
            b'M' => Ok(MidpointPeg),
            b'R' => Ok(PrimaryPeg),
            b'Q' => Ok(MarketMakerPeg),
            b'm' => Ok(Midpoint),

            _ => BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "PriceType".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Limit           => b'L',
            MarketPeg       => b'P',
            MidpointPeg     => b'M',
            PrimaryPeg      => b'R',
            MarketMakerPeg  => b'Q',
            Midpoint        => b'm',
        }
    }

}

