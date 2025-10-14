
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
    BrokenReason,
    OrderToken
};

use crate::msg::options::*;


/// Informs you that an execution has been broken.
/// The trade is no longer good and will not clear.
///
/// You will only receive a BrokenTrade after first receiving an 
/// OrderExecuted message for a given order.
#[derive(Debug, Clone)]
pub struct BrokenTrade {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    match_number: u64,
    reason: BrokenReason,
    order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl BrokenTrade {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 35 {
            return Err(OuchError::Parse("BrokenTrade".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            match_number: u64_from_be_bytes(&data[12..=19])?,
            reason: BrokenReason::parse(data[20])?,
            order_token: OrderToken::parse(&data[21..=34])?,
            optional_appendage: if data.len() >= 35 {
                OptionalAppendage::parse(&data[35..])?
            } else {
                OptionalAppendage::new()
            }
        })
    }
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Match Number being broken (as transmitted in the OrderExecuted response).
    pub fn match_number(&self) -> u64 { self.match_number }

    /// Refers to the order whose execution has been broken.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// Reason the trade execution was broken.
    pub fn reason(&self) -> BrokenReason { self.reason }

    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_token(&self) -> &OrderToken { &self.order_token }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}

