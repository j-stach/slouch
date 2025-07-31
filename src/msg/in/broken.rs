
use serde::{ Deserialize, Serialize };
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokenTrade {
    pub match_number: u64,
    pub reason: char,
}

impl BrokenTrade {

    pub(super) fn parse(data: &[u8]) -> Result<Self, String> {

        if data.len() < 9 {
            return Err("BrokenTrade: insufficient data".into());
        }

        Ok(BrokenTrade {
            match_number: u64::from_be_bytes(data[0..8].try_into().unwrap()),
            reason: data[8] as char,
        })
    }

}

