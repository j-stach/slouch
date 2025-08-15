
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::{ 
    helper::{ check_string_compliance, encode_fixed_str }, 
    error::BadElementError 
};

/// Strong type for stock symbols that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct StockSymbol(String);

impl StockSymbol {

    /// Generate a new StockSymbol from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_string_compliance(s, 8, "StockSymbol")?;

        Ok(StockSymbol(s.to_string()))
    }

    /// StockSymbol should have its length checked when it is created.
    /// This method will encode it into a fixed length of 8 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 8)
    }
}


