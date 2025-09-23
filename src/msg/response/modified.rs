
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
    OrderRefNum,
    OrderToken
};


/// 
#[derive(Debug)]
pub struct OrderModified {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub side: Side,
    pub quantity: u32,
    optional_appendage: OptionalAppendage
}

impl OrderModified {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo![]
    }

}
