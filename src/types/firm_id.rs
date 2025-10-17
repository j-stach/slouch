
use std::fmt;

use crate::{ 
    helper::{ 
        check_alpha_uppercase,
        ascii_from_utf8,
        fixed_str_4,
    }, 
    error::BadElementError 
};

/// Strong type for firm IDs that ensures protocol compliance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirmId([u8; 4]);

impl FirmId {

    /// Generate a new FirmId from a protocol-compliant string.
    pub fn from(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();

        if s.len() != 4 {
            return Err(
                BadElementError::WrongSize("FirmId".to_string(), 4, s.len())
            )
        }

        check_alpha_uppercase(s, "FirmId")?;

        Ok(FirmId(fixed_str_4(s)))
    }

    /// Get the FirmId as a string, ignoring trailing spaces.
    pub fn to_string(&self) -> String { 
        // TODO: 
        self.0.iter()
            .map(|b| *b as char)
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    // FirmId should have its length checked when it is created.
    // This method will encode it into a fixed length of 4 bytes.
    pub(crate) fn encode(&self) -> [u8; 4] {
        self.0
    }

    // Assumes the strings from NASDAQ are compliant.
    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self::from(ascii_from_utf8(data)?)?)
    }
}

impl fmt::Display for FirmId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

impl Default for FirmId {
    fn default() -> Self {
        FirmId(*b"    ")
    }
}

