
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrokenReason {

    /// Trade deemed "clearly erroneous."
    Erroneous,

    /// Both parties agreed to break the trade.
    Consent,

    /// Manually broken by NASDAQ supervisory terminal.
    Supervisory,

    /// Broken by external third party.
    External

}

impl BrokenReason {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'E' => Ok(Erroneous),
            b'C' => Ok(Consent),
            b'S' => Ok(Supervisory),
            b'X' => Ok(External),

            _ => BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "BrokenReason".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Erroneous   => b'E',
            Consent     => b'C',
            Supervisory => b'S',
            External    => b'X',
        }
    }

}
