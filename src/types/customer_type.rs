
use crate::error::BadElementError;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomerType {

    /// Retail Designated Order 
    Retail,

    /// Not Retail Designated (default)
    NotRetail
}

impl CustomerType {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use CustomerType::*;
        match data {
            b'R' => Ok(Retail),
            b'N' => Ok(NotRetail),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "CustomerType".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use CustomerType::*;
        match self {
            Retail => b'R',
            NotRetail  => b'N',
        }
    }

}
