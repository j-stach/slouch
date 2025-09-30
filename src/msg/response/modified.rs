
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
    Side,
    StockSymbol, 
    Price,
    TimeInForce,
    Display,
    Capacity,
    CrossType,
    OrderState,
    OrderRefNum,
    OrderToken
};


/// 
#[derive(Debug)]
pub struct OrderModified {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub side: Side,
    pub quantity: u32,
    optional_appendage: OptionalAppendage
}

impl OrderModified {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 20 {
            return Err(OuchError::Parse("OrderModified".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            side: Side::parse(data[12])?, 
            quantity: u32_from_be_bytes(&data[13..=16])?, 
            optional_appendage: OptionalAppendage::parse(&data[18..])?
        })
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}
