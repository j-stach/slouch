
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

use crate::msg::options::OptionalAppendage;


///
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

        if data.len() < 38 {
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
            order_token: OrderToken::parse(&data[21..=35])?,
            optional_appendage: OptionalAppendage::parse(&data[36..])?
        })
    }
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    pub fn match_number(&self) -> u64 { self.match_number }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    pub fn reason(&self) -> BrokenReason { self.reason }

    pub fn order_token(&self) -> &OrderToken { &self.order_token }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}

