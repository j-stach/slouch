
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
    OrderToken
};


/// 
#[derive(Debug)]
pub struct OrderPriorityUpdate {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub price: Price,
    pub display: Display,
    pub order_ref_num: u64,
    optional_appendage: OptionalAppendage
}

impl OrderPriorityUpdate {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 31 {
            return Err(OuchError::Parse("OrderPriorityUpdate".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            price: Price::parse(&data[12..=19])?,
            display: Display::parse(data[20])?,
            order_ref_num: u64_from_be_bytes(&data[21..=28])?,
            optional_appendage: OptionalAppendage::parse(&data[29..])?
        })
    }

}


/// 
#[derive(Debug)]
pub struct OrderRestated {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub reason: RestateReason,
    optional_appendage: OptionalAppendage
}

impl OrderRestated {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 63 {
            return Err(OuchError::Parse("OrderAccepted".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            reason: RestateReason::parse(&data[12])?,
            optional_appendage: OptionalAppendage::parse(&data[13..])?
        })
    }

}
