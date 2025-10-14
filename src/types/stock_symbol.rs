
use std::fmt;

use crate::{ 
    helper::{ 
        check_alpha_compliance, 
        ascii_from_utf8,
        fixed_str_8
    }, 
    error::BadElementError 
};

/// Strong type for stock symbols that ensures protocol compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StockSymbol([u8; 8]);

impl StockSymbol {

    /// Generate a new StockSymbol from a protocol-compliant string.
    pub fn from(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_alpha_compliance(s, 8, "StockSymbol")?;

        Ok(StockSymbol(fixed_str_8(s)))
    }

    /// When Symbol is optional, you can use this to leave it blank.
    pub fn blank() -> Self {
        Self::from(String::new())
            .expect("Creates blank StockSymbol from empty string")
    }

    /// Get the StockSymbol as a string, ignoring trailing spaces.
    pub fn to_string(&self) -> String { 
        self.0.iter()
            .map(|b| *b as char)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    // StockSymbol should have its length checked when it is created.
    // This method will encode it into a fixed length of 8 bytes.
    pub(crate) fn encode(&self) -> [u8; 8] {
        self.0
    }

    // Assumes the strings from NASDAQ are compliant.
    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self::from(ascii_from_utf8(data)?)?)
    }
}

impl fmt::Display for StockSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

