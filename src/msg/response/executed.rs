
use serde::{ Deserialize, Serialize };

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
    LiquidityFlag,
    MatchNumber,
    OrderToken
};


///
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderExecuted {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub quantity: u32,
    pub price: Price,
    pub liquidity_flag: LiquidityFlag,
    pub match_number: MatchNumber,
    optional_appendage: OptionalAppendage
}

impl OrderExecuted {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        /*
        if data.len() < 30 {
            return Err(OuchError::Parse("OrderExecuted".to_string()))
        }

        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        let executed_shares = u32_from_be_bytes(&data[14..18])?;
        let execution_price = u32_from_be_bytes(&data[18..22])?;
        let match_number = u64_from_be_bytes(&data[22..30])?;

        Ok(OrderExecuted {
            order_token,
            executed_shares,
            execution_price,
            match_number
        })
        */
        todo!{}
    }

}

