
use crate::error::BadElementError;

///
pub enum CustomerType {

    /// Retail Designated Order 
    Retail,

    /// Not Retail Designated (default)
    NotRetail
}

impl CustomerType {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use Self::*;
        match data {
            b'R' => Ok(Retail),
            b'N' => Ok(NotRetail),

            _ => BadElementError::InvalidEnum(
                data as char, 
                "CustomerType".to_string()
            )
        }
    }

    pub(crate) fn encode(&self) -> u8 {
        
        use Self::*;
        match self {
            Retail => b'R',
            NotRetail  => b'N',
        }
    }

}
