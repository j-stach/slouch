
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
pub struct OrderPriorityUpdate {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub price: Price,
    pub display: Display,
    pub order_ref_num: OrderRefNum,
    optional_appendage: OptionalAppendage
}

impl OrderPriorityUpdate {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo![]
    }

}


/// 
#[derive(Debug)]
pub struct OrderRestated {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub side: Side,
    pub quantity: u32,
    optional_appendage: OptionalAppendage
}

impl OrderRestated {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo![]
    }

}
