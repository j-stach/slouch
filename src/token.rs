
// OrderToken type to check for bytelength

use serde::{ Deserialize, Serialize };
use std::fmt;

use crate::helper::encode_fixed_str;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderToken(String);

impl OrderToken {

    pub fn new(s: impl AsRef<str>) -> Result<Self, String> {
        let s = s.as_ref();
        if s.len() != 14 {
            return Err(format!("OrderToken must be 14 characters, got {}", s.len()));
        }
        if !s.chars().all(|c| c.is_ascii()) {
            return Err("OrderToken must be ASCII".to_string());
        }
        Ok(OrderToken(s.to_string()))
    }

    pub fn from_parts(prefix: &str, counter: u64) -> Result<Self, String> {
        let s = format!("{}{:0>width$}", prefix, counter, width = 14 - prefix.len());
        OrderToken::new(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// OrderToken should have its length checked when it is created.
    /// This method will encode it into a fixed length of 14 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(self.as_str(), 14)
    }
}

impl fmt::Display for OrderToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
