
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
};

use crate::msg::options::*;


/// 
#[derive(Debug, Clone)]
pub struct OrderModified {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
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
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> Side { self.side }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}
