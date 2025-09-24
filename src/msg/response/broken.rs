
use chrono::NaiveTime;
use serde::{ Deserialize, Serialize };

use crate::{
    error::OuchError,
    options::OptionalAppendage,
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
    MatchNumber,
    OrderToken
};


///
#[derive(Debug, Serialize, Deserialize)]
pub struct BrokenTrade {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub match_number: u64,
    pub broken_reason: BrokenReason,
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl BrokenTrade {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        todo!{}
    }

}

