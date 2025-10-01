
use std::fmt;

use crate::{ 
    helper::{ 
        check_alpha_compliance,
        encode_fixed_str,
        ascii_from_utf8,
    }, 
    error::BadElementError,
};

/// Strong type for broker IDs that ensures protocol compliance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokerId(String);

impl BrokerId {

    /// Generate a new BrokerId from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_alpha_compliance(s, 4, "BrokerId")?;

        Ok(BrokerId(s.to_string()))
    }

    // BrokerId should have its length checked when it is created.
    // This method will encode it into a fixed length of 4 bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&self.0, 4)
    }

    // Assumes the strings from NASDAQ are compliant.
    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self(ascii_from_utf8(data)?))
    }
}

impl fmt::Display for BrokerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

