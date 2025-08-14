
use std::fmt;
use serde::{ Deserialize, Serialize };
use derive_more::{ Deref, DerefMut };

use crate::{
    helper::encode_fixed_str,
    error::{ OuchError, BadElementError }
};


/// Strong type for order tokens that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderToken(String);

impl OrderToken {

    /// Generate a new OrderToken from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, OuchError> {

        let s = s.as_ref();

        // Check length
        if s.len() != 14 {
            return Err(
                BadElementError::OrderToken(
                    format!("Must be 14 characters, got {}", s.len())
                ).into()
            );
        }

        // Check ASCII
        if !s.chars().all(|c| c.is_ascii()) {
            return Err(
                BadElementError::OrderToken("Must be ASCII".to_string())
                    .into()
            );
        }

        Ok(OrderToken(s.to_string()))
    }

    /*
    // TODO Move to OrderTokenGenerator, etc.
    // Needs to convert u64 into number based on Client 
    /// 
    pub fn from_parts(prefix: &str, counter: u64) -> Result<Self, OuchError> {
        let s = format!("{}{:0>width$}", prefix, counter, width = 14 - prefix.len());
        OrderToken::new(s)
    }
    */

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

