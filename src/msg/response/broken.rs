
use serde::{ Deserialize, Serialize };
use crate::{
    error::OuchError,
    helper::u64_from_be_bytes,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokenTrade {
    pub match_number: u64,
    pub reason: char,
}

impl BrokenTrade {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 9 {
            return Err(OuchError::Parse("BrokenTrade".to_string()))
        }

        Ok(BrokenTrade {
            match_number: u64_from_be_bytes(&data[0..8])?,
            // TODO Enum
            reason: data[8] as char,
        })
    }

}

