
use std::fmt;

use crate::{ 
    helper::{ 
        check_alpha_uppercase,
        ascii_from_utf8,
        encode_fixed_str 
    }, 
    error::BadElementError 
};

/// Strong type for firm IDs that ensures protocol compliance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmId(String);

impl FirmId {

    /// Generate a new FirmId from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();

        if s.len() != 4 {
            return Err(
                BadElementError::WrongSize("FirmId".to_string(), 4, s.len())
            )
        }

        check_alpha_uppercase(s, "FirmId")?;

        Ok(FirmId(s.to_string()))
    }

    // FirmId should have its length checked when it is created.
    // This method will encode it into a fixed length of 4 bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&self.0, 4)
    }

    // Assumes the strings from NASDAQ are compliant.
    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self(ascii_from_utf8(data)?))
    }
}

impl fmt::Display for FirmId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

