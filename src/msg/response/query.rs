
use chrono::NaiveTime;

use crate::{
    error::OuchError,
    helper::{
        u64_from_be_bytes,
        nanosec_from_midnight
    },
    types::UserRefNum,
    msg::options::OptionalAppendage,
};


/// 
#[derive(Debug, Clone)]
pub struct AccountQueryResponse {
    timestamp: NaiveTime,
    next_user_ref_num: UserRefNum,
    optional_appendage: OptionalAppendage
}

impl AccountQueryResponse {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 14 {
            return Err(OuchError::Parse("AccountQueryResponse".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            next_user_ref_num: UserRefNum::parse(&data[8..=11])?,
            optional_appendage: OptionalAppendage::parse(&data[12..])?
        })
    }
    
    pub fn timestamp(&self) -> &NaiveTime { &self.timestamp }
    
    /// Gets the user reference number.
    pub fn next_user_ref_num(&self) -> &UserRefNum {
        &self.next_user_ref_num
    }

    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}
