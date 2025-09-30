
use crate::error::BadElementError;


///  
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capacity {
    Agency,
    Principal,
    Riskless,
    Other
}

impl Capacity {

    pub(crate) fn encode(&self) -> u8 {

        use Self::*;
        match self {
            Agency => b'A',
            Principal => b'P',
            Riskless => b'R',
            Other => b'O',
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'A' => Ok(Agency),
            b'P' => Ok(Principal),
            b'R' => Ok(Riskless),
            b'O' => Ok(Other),

            _ => BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "Capacity".to_string()
            )
        }
    }
}

