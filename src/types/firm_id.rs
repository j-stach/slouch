
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::{ 
    helper::encode_fixed_str, 
    error::BadElementError 
};

/// Strong type for firm IDs that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct FirmId(String);

impl FirmId {

    /// Generate a new FirmId from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();

        if s.len() != 4 {
            return Err(
                BadElementError::WrongSize("FirmId".to_string(), 4, s.len())
            );
        }

        if !s.chars().all(|c| c.is_ascii()) {
            return Err(
                BadElementError::NotAscii("FirmId".to_string())
            );
        }

        Ok(FirmId(s.to_string()))
    }

    /// FirmId should have its length checked when it is created.
    /// This method will encode it into a fixed length of 4 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 4)
    }
}

