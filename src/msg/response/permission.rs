
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
    FirmId,
    Display,
    Capacity,
    CrossType,
    OrderState,
    OrderRefNum,
    OrderToken
};


/// 
#[derive(Debug, Clone)]
pub struct DisableOrderEntryResponse {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
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
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}


/// 
#[derive(Debug, Clone)]
pub struct EnableOrderEntryResponse {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
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
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}
