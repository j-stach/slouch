
use chrono::NaiveTime;

use crate::{
    error::OuchError,
    helper::{ 
        u32_from_be_bytes, 
        u64_from_be_bytes,
        nanosec_from_midnight
    }
};

use crate::types::{ 
    UserRefNum,
    Price,
    LiquidityFlag,
    MatchNumber,
};


///
#[derive(Debug, Clone)]
pub struct OrderExecuted {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub quantity: u32,
    pub price: Price,
    pub liquidity_flag: LiquidityFlag,
    pub match_number: u64,
    optional_appendage: OptionalAppendage
}

impl OrderExecuted {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 35 {
            return Err(OuchError::Parse("OrderExecuted".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            quantity: u32_from_be_bytes(&data[12..=15])?, 
            price: Price::parse(&data[17..=23])?,
            liquidity_flag: LiquidityFlag::parse(&data[24])?,
            match_number: u64_from_be_bytes(&data[25..=32])?,
            optional_appendage: OptionalAppendage::parse(&data[33..])?
        })
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}

