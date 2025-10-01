
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
    FirmId,
};

use crate::msg::options::OptionalAppendage;


/// 
#[derive(Debug, Clone)]
pub struct DisableOrderEntryResponse {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    firm: FirmId,
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntryResponse {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 18 {
            return Err(OuchError::Parse("DisableOrderEntryResponse".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            firm: FirmId::parse(&data[12..=15])?,
            optional_appendage: OptionalAppendage::parse(&data[16..])?
        })
    }
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> &FirmId { &self.firm }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}


/// 
#[derive(Debug, Clone)]
pub struct EnableOrderEntryResponse {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    firm: FirmId,
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntryResponse {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 18 {
            return Err(OuchError::Parse("EnableOrderEntryResponse".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            firm: FirmId::parse(&data[12..=15])?,
            optional_appendage: OptionalAppendage::parse(&data[16..])?
        })
    }
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum {
        self.user_ref_num
    }

    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> &FirmId { &self.firm }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}
