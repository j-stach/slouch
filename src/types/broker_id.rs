
use std::fmt;

use crate::{ 
    helper::{ 
        check_alpha_compliance,
        fixed_str_4,
        ascii_from_utf8,
    }, 
    error::BadElementError,
};

/// Strong type for broker IDs that ensures protocol compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrokerId([u8; 4]);

impl BrokerId {

    /// Generate a new BrokerId from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_alpha_compliance(s, 4, "BrokerId")?;

        Ok(BrokerId(fixed_str_4(s)))
    }

    /// Get the BrokerId as a string, ignoring trailing spaces.
    pub fn to_string(&self) -> String { 
        // TODO: 
        self.0.iter()
            .map(|b| *b as char)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    // BrokerId should have its length checked when it is created.
    // This method will encode it into a fixed length of 4 bytes.
    pub(crate) fn encode(&self) -> [u8; 4] {
        self.0
    }

    // Assumes the strings from NASDAQ are compliant.
    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self::new(ascii_from_utf8(data)?)?)
    }
}

impl fmt::Display for BrokerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

