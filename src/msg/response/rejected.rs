
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
    RejectReason,
    OrderToken
};

use crate::msg::options::*;


/// May be sent in response to an EnterOrder or ReplaceOrder request if the 
/// order or replace cannot be accepted at this time.
///
/// NOTE: The UserRefNum of a Rejected Message cannot be re-used.
#[derive(Debug, Clone)]
pub struct OrderRejected {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    reason: RejectReason,
    order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl OrderRejected {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 28 {
            return Err(OuchError::Parse("OrderRejected".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            reason: RejectReason::parse(&data[12..=13])?,
            order_token: OrderToken::parse(&data[14..=27])?,
            optional_appendage: if data.len() >= 28 {
                OptionalAppendage::parse(&data[28..])?
            } else {
                OptionalAppendage::new()
            }
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Refers to the order being rejected.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// Reason why the order was rejected.
    pub fn reason(&self) -> RejectReason { self.reason }
    
    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_token(&self) -> &OrderToken { &self.order_token }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}
