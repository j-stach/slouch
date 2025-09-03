
use crate::error::BadElementError;

///
pub enum Display {
    Visible,
    Hidden,
    Attributable,
}

impl Display {
    
    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Visible => b'Y',
            Hidden => b'N',
            Attributable => b'A',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'Y' => Ok(Visible),
            b'N' => Ok(Hidden),
            b'A' => Ok(Attributable),
            _ => BadElementError::InvalidEnum(
                data as char, 
                "Display".to_string()
            )
        }
    }
}

