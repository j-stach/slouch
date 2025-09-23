
// query::AccountQueryResponse

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
pub struct AccountQueryResponse {
    pub timestamp: NaiveTime,
    pub next_user_ref_num: UserRefNum,
    optional_appendage: OptionalAppendage
}

impl AccountQueryResponse {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo![]
    }

}
