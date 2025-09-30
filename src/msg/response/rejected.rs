
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
    RejectedReason,
    OrderToken
};


///
#[derive(Debug, Clone)]
pub struct OrderRejected {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub rejected_reason: RejectedReason,
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl OrderRejected {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 30 {
            return Err(OuchError::Parse("OrderRejected".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            rejected_reason: RejectedReason::parse(&data[12..=13])?,
            order_token: OrderToken::parse(&data[14..=27])?,
            optional_appendage: OptionalAppendage::parse(&data[28..])?
        })
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}
