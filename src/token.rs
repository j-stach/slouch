
// OrderToken type to check for bytelength

use serde::{ Deserialize, Serialize };
use std::fmt;

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
}

impl fmt::Display for OrderToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
