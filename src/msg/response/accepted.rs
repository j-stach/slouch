
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
pub struct OrderAccepted {
    pub timestamp: NaiveTime,
    pub user_ref_num: UserRefNum,
    pub side: Side,
    pub quantity: u32,
    pub symbol: StockSymbol,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub order_ref_num: u64,
    pub capacity: Capacity,
    pub intermarket_sweep_eligibility: bool,
    pub cross_type: CrossType,
    pub order_state: OrderState
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl OrderAccepted {

    // TODO
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        /*
        if data.len() < 39 {
            return Err(OuchError::Parse("OrderAccepted".to_string()))
        }

        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        let ts = u64_from_be_bytes(&data[14..22])?;
        let timestamp = nanosec_from_midnight(ts);

        let shares = u32_from_be_bytes(&data[22..26])?;

        let stock_symbol = StockSymbol::new(
            String::from_utf8_lossy(&data[26..34])
                .trim_end()
                .to_string()
        )?;

        let price = u32_from_be_bytes(&data[34..38])?;

        // TODO Enum
        let buy_sell_indicator = data[38] as char;

        Ok(OrderAccepted {
            order_token,
            timestamp,
            shares,
            stock_symbol,
            price,
            buy_sell_indicator,
        })
        */

        todo![]
    }

}
