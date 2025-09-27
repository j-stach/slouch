
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::{ 
    helper::{ 
        check_string_compliance,
        check_string_alpha,
        encode_fixed_str 
    }, 
    error::BadElementError 
};

/// Strong type for broker IDs that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct BrokerId(String);

impl BrokerId {

    /// Generate a new BrokerId from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_string_compliance(s, 4, "BrokerId")?;
        check_string_alpha(s, "BrokerId")?;

        Ok(FirmId(s.to_string()))
    }

    /// BrokerId should have its length checked when it is created.
    /// This method will encode it into a fixed length of 4 bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 4)
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, BadElementError> {

        // TODO
        todo!{}
    }
}

