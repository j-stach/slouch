
use chrono::NaiveTime;
use serde::{ Deserialize, Serialize };

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
    RejectedReason,
    OrderToken
};


///
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRejected {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub rejected_reason: RejectedReason,
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl OrderRejected {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        /*
        if data.len() < 15 {
            return Err(OuchError::Parse("OrderRejected".to_string()))
        }
        
        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        // TODO Enum
        let reason = data[14] as char;

        Ok(OrderRejected {
            order_token,
            reason
        })
        */
        todo!{}
    }

}
