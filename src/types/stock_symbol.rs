
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::{ 
    helper::encode_fixed_str, 
    error::BadElementError 
};

/// Strong type for stock symbols that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct StockSymbol(String);

impl StockSymbol {

    /// Generate a new StockSymbol from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();

        if s.len() != 8 {
            return Err(
                BadElementError::WrongSize("StockSymbol".to_string(), 8, s.len())
            );
        }

        if !s.chars().all(|c| c.is_ascii()) {
            return Err(
                BadElementError::NotAscii("StockSymbol".to_string())
            );
        }

        Ok(StockSymbol(s.to_string()))
    }

    /// StockSymbol should have its length checked when it is created.
    /// This method will encode it into a fixed length of 8 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 8)
    }
}


