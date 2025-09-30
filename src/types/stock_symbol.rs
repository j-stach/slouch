
use std::fmt;

use crate::{ 
    helper::{ 
        check_alpha_compliance, 
        ascii_from_utf8,
        encode_fixed_str 
    }, 
    error::BadElementError 
};

/// Strong type for stock symbols that ensures protocol compliance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockSymbol(String);

impl StockSymbol {

    /// Generate a new StockSymbol from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_alpha_compliance(s, 8, "StockSymbol")?;

        Ok(StockSymbol(s.to_string()))
    }

    /// When Symbol is optional, you can use this to leave it blank.
    pub fn blank() -> Self {
        Self::new(String::new())
            .expect("Creates blank StockSymbol from empty string")
    }

    // StockSymbol should have its length checked when it is created.
    // This method will encode it into a fixed length of 8 bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 8)
    }

    // Assumes the strings from NASDAQ are compliant.
    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self(ascii_from_utf8(data)?))
    }
}

impl fmt::Display for StockSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

