
use std::fmt;

use crate::{
    helper::{ 
        ascii_from_utf8,
        fixed_str_14
    },
    error::BadElementError
};


/// Strong type for `ClOrdId` that ensures protocol compliance.
/// Can be used to differentiate strategies, etc.
/// Will not be checked for day-uniqueness for each OUCH account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderToken([u8; 14]);

impl OrderToken {

    /// Generate a new OrderToken from a protocol-compliant string.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();

        if s.len() > 14  {
            return Err(
                BadElementError::WrongSize(
                    "OrderToken".to_string(), 14, s.len())
            );
        }

        // OrderToken is allowed to contain spaces and numbers
        if !s.chars().all(|c| {
            c.is_ascii_alphanumeric() || c == ' '
        }) {
            return Err(
                BadElementError::NotAlpha("OrderToken".to_string())
            );
        }

        Ok(OrderToken(fixed_str_14(s)))
    }

    /// Get the OrderToken as a string, ignoring trailing spaces.
    pub fn to_string(&self) -> String { 
        // TODO: 
        self.0.iter()
            .map(|b| *b as char)
            .collect::<String>()
            .trim_end()
            .to_string()
    }


    // OrderToken should have its length checked when it is created.
    // This method will encode it into a fixed length of 14 bytes.
    pub(crate) fn encode(&self) -> [u8; 14] {
        self.0
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        Ok(Self::new(ascii_from_utf8(data)?)?)
    }
}

impl fmt::Display for OrderToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

