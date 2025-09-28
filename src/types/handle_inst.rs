
use crate::error::BadElementError;

///
pub enum HandleInst {

    /// Imbalance only (CrossType::OpeningCross & CrossType::ClosingCross)
    ImbalanceOnly,

    /// Retail order type 1 (CrossType::Retail)
    RetailOrder1,

    /// Retail order type 2 (CrossType::Retail)
    RetailOrder2,

    /// Retail price improvement (CrossType::Retail)
    RetailPriceImprovement,

    /// Extended Life + Continuous (CrossType::ExtendedLife)
    ExtendedContinuous,

    /// Direct Listing Capital Raise (CrossType::Halt)
    DirectListingCapitalRaise,

    /// No handling instructions.
    NoInstructions
}

impl HandleInst {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'I' => Ok(ImbalanceOnly),
            b'O' => Ok(RetailOrder1),
            b'T' => Ok(RetailOrder2),
            b'Q' => Ok(RetailPriceImprovement),
            b'B' => Ok(ExtendedContinuous),
            b'D' => Ok(DirectListingCapitalRaise),
            b' ' => Ok(NoInstructions),

            _ => BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "HandleInst".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
             ImbalanceOnly              => b'I',
             RetailOrder1               => b'O',
             RetailOrder2               => b'T',
             RetailPriceImprovement     => b'Q',
             ExtendedContinuous         => b'B',
             DirectListingCapitalRaise  => b'D',
             NoInstructions             => b' ',
        }
    }

}

