
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::helper::encode_fixed_str;

#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct FirmId(String);

impl FirmId {

    pub fn new(s: impl AsRef<str>) -> Result<Self, String> {

        let s = s.as_ref();

        if s.len() != 4 {
            return Err(format!("FirmId must be 4 characters, got {}", s.len()));
        }

        if !s.chars().all(|c| c.is_ascii()) {
            return Err("FirmId must be ASCII".to_string());
        }

        Ok(FirmId(s.to_string()))
    }

    /// FirmId should have its length checked when it is created.
    /// This method will encode it into a fixed length of 4 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 4)
    }
}

#[derive(Debug, Deref, Clone, DerefMut, Serialize, Deserialize)]
pub struct StockSymbol(String);

impl StockSymbol {

    pub fn new(s: impl AsRef<str>) -> Result<Self, String> {

        let s = s.as_ref();

        if s.len() != 8 {
            return Err(format!("StockSymbol must be 8 characters, got {}", s.len()));
        }

        if !s.chars().all(|c| c.is_ascii()) {
            return Err("StockSymbol must be ASCII".to_string());
        }

        Ok(StockSymbol(s.to_string()))
    }

    /// StockSymbol should have its length checked when it is created.
    /// This method will encode it into a fixed length of 8 bytes.
    pub fn encode(&self) -> Vec<u8> {
        encode_fixed_str(&*self, 8)
    }
}


