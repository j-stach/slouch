
use std::fmt;

use crate::{
    helper::{ 
        check_alpha_compliance, 
        ascii_from_utf8,
        encode_fixed_str 
    },
    error::BadElementError
};


/// Strong type for `ClOrdId` that ensures protocol compliance.
/// Will not be checked for day-uniqueness for each OUCH account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderToken(String);

impl OrderToken {

    /// Generate a new OrderToken from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_alpha_compliance(s, 14, "OrderToken")?;

        Ok(OrderToken(s.to_string()))
    }

    /// Get the OrderToken as a string.
    pub fn as_str(&self) -> &str { &*self }

    // OrderToken should have its length checked when it is created.
    // This method will encode it into a fixed length of 14 bytes.
    pub(crate) fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 14)
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        ascii_from_utf8(data)
    }
}

impl fmt::Display for OrderToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

