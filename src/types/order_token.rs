
use std::fmt;
use serde::{ Deserialize, Serialize };
use derive_more::{ Deref, DerefMut };

use crate::{
    helper::{ check_string_compliance, encode_fixed_str },
    error::BadElementError
};


/// Strong type for order tokens that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderToken(String);

impl OrderToken {

    /// Generate a new OrderToken from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_string_compliance(s, 14, "OrderToken")?;

        Ok(OrderToken(s.to_string()))
    }

    /// Get the OrderToken as a string.
    pub fn as_str(&self) -> &str { &*self }

    /// OrderToken should have its length checked when it is created.
    /// This method will encode it into a fixed length of 14 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 14)
    }
}

impl fmt::Display for OrderToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

