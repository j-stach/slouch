
use std::fmt;
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::{ 
    helper::{ 
        check_alpha_compliance,
        ascii_from_utf8,
        encode_fixed_str 
    }, 
    error::BadElementError 
};

/// Strong type for route IDs that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct RouteId(String);

impl RouteId {

    /// Generate a new RouteId from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_alpha_compliance(s, 4, "RouteId")?;

        Ok(RouteId(s.to_string()))
    }

    // RouteId should have its length checked when it is created.
    // This method will encode it into a fixed length of 4 bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 4)
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        ascii_from_utf8(data)
    }
}

impl fmt::Display for RouteId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

