
use crate::error::BadElementError;

/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Visible,
    Hidden,
    Attributable,
}

impl Display {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use Display::*;
        match self {
            Visible => b'Y',
            Hidden => b'N',
            Attributable => b'A',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Display::*;
        match data {
            b'Y' => Ok(Visible),
            b'N' => Ok(Hidden),
            b'A' => Ok(Attributable),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "Display".to_string()
            ))
        }
    }
}

