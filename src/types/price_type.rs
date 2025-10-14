
use crate::error::BadElementError;

/// Defines how the order's Price will be interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceType {

    /// Standard limit order with a fixed price set by the trader. 
    /// The order executes only at or better than the specified price.
    Limit,

    /// Specifies a pegged order that tracks the best bid (for sell orders) or 
    /// best offer (for buy orders) of the National Best Bid and Offer (NBBO), 
    /// typically with an optional offset.
    MarketPeg,

    /// Specifies a pegged order that tracks the midpoint of the NBBO. 
    /// The order is typically non-displayed and executes at the midpoint, 
    /// offering price improvement.
    MidpointPeg,

    /// Specifies a pegged order that tracks the primary market’s quote, 
    /// often with an offset. 
    /// Unlike MarketPeg, it focuses on the primary exchange’s quote 
    /// rather than the consolidated NBBO.
    PrimaryPeg,

    /// Specifies a pegged order designed for market makers, 
    /// typically tracking a reference price (e.g., NBBO or primary quote) 
    /// with constraints to meet market maker obligations 
    /// (e.g., maintaining two-sided quotes).
    /// Not usable with `OptionValue::DiscretionPriceType`.
    MarketMakerPeg,

    /// Specifies a static (not pegged) midpoint order that executes 
    /// at the NBBO midpoint price at the time of order entry.
    /// Not usable with `OptionValue::DiscretionPriceType`.
    // TODO: Clarify this with devs 
    Midpoint
}

impl PriceType {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use PriceType::*;
        match data {
            b'L' => Ok(Limit),
            b'P' => Ok(MarketPeg),
            b'M' => Ok(MidpointPeg),
            b'R' => Ok(PrimaryPeg),
            b'Q' => Ok(MarketMakerPeg),
            b'm' => Ok(Midpoint),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "PriceType".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use PriceType::*;
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

