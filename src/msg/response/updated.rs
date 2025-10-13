
use chrono::NaiveTime;

use crate::{
    error::OuchError,
    helper::{ 
        u64_from_be_bytes,
        nanosec_from_midnight
    }
};

use crate::types::{ 
    UserRefNum,
    Price,
    Display,
    RestateReason
};

use crate::msg::options::*;


/// Sent whenever priority of the order has been changed by the system.
#[derive(Debug, Clone)]
pub struct OrderPriorityUpdate {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    price: Price,
    display: Display,
    order_ref_num: u64,
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
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// The limit price of the order.
    pub fn price(&self) -> Price { self.price }

    /// The new display for the order.
    pub fn display(&self) -> Display { self.display }

    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_ref_num(&self) -> u64 { self.order_ref_num }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}


/// Sent when the system has modified an order as part of its order management.
#[derive(Debug, Clone)]
pub struct OrderRestated {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    reason: RestateReason,
    optional_appendage: OptionalAppendage
}

impl OrderRestated {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 15 {
            return Err(OuchError::Parse("OrderRestated".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            reason: RestateReason::parse(data[12])?,
            optional_appendage: OptionalAppendage::parse(&data[13..])?
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// Reason for restating the order.
    pub fn reason(&self) -> RestateReason { self.reason }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}
