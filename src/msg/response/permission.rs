
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
#[derive(Debug)]
pub struct DisableOrderEntryResponse {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntryResponse {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo![]
    }

}


/// 
#[derive(Debug)]
pub struct EnableOrderEntryResponse {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntryResponse {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo![]
    }

}
