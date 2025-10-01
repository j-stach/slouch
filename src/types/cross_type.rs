
use crate::error::BadElementError;

/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossType {

    ///
    ContinuousMarket,

    /// NASDAQ Only
    OpeningCross,

    /// NASDAQ Only
    ClosingCross,

    /// "Halt/IPO", NASDAQ Only
    Halt,

    /// NASDAQ Only
    Supplemental,

    /// BX Only
    Retail,

    /// NASDAQ Only
    ExtendedLife,

    /// NASDAQ Only
    AfterHoursClose,
}

impl CrossType {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use CrossType::*;
        match self {
            ContinuousMarket => b'N',
            OpeningCross     => b'O',
            ClosingCross     => b'C',
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
            b'O' => Ok(OpeningCross),
            b'C' => Ok(ClosingCross),
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

