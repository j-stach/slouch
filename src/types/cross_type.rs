
use crate::error::BadElementError;

/// Designates the specific market session, auction, 
/// or execution mechanism the order should participate in, 
/// influencing when and how it executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossType {

    /// Standard limit order for ongoing matching in the regular trading book.
    /// (NASDAQ Only)
    ContinuousMarket,

    /// Participates in auction to determine the official opening price.
    /// (NASDAQ Only)
    Opening,

    /// Joins the closing auction for the official close.
    /// (NASDAQ Only)
    Closing,

    /// "Halt/IPO"; for reopenings after halts or initial public offerings.
    /// (NASDAQ Only)
    Halt,

    /// Provides additional liquidity during crosses.
    /// (NASDAQ Only)
    Supplemental,

    /// Specific to BX exchange for retail-designated orders.
    /// (BX Only)
    Retail,

    /// For orders with longer persistence.
    /// (NASDAQ Only)
    ExtendedLife,

    /// For after-hours closing mechanisms.
    /// (NASDAQ Only)
    AfterHoursClose,
}

impl CrossType {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use CrossType::*;
        match self {
            ContinuousMarket => b'N',
            Opening          => b'O',
            Closing          => b'C',
            Halt             => b'H',
            Supplemental     => b'S',
            Retail           => b'R',
            ExtendedLife     => b'E',
            AfterHoursClose  => b'A',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use CrossType::*;
        match data {
            b'N' => Ok(ContinuousMarket),
            b'O' => Ok(Opening),
            b'C' => Ok(Closing),
            b'H' => Ok(Halt),
            b'S' => Ok(Supplemental),
            b'R' => Ok(Retail),
            b'E' => Ok(ExtendedLife),
            b'A' => Ok(AfterHoursClose),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "CrossType".to_string()
            ))
        }
    }
}

