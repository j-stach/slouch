
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
pub struct OrderReplaced {
    pub timestamp: NaiveTime,
    pub old_user_ref_num: UserRefNum,
    pub new_user_ref_num: UserRefNum,
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

impl OrderReplaced {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 67 {
            return Err(OuchError::Parse("OrderReplaced".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..8])?;
                nanosec_from_midnight(ts)
            },
            old_user_ref_num: UserRefNum::parse(&data[8..=11])?,
            new_user_ref_num: UserRefNum::parse(&data[12..=15])?,
            side: Side::parse(data[16])?, 
            quantity: u32_from_be_bytes(&data[17..=20])?, 
            symbol: StockSymbol::parse(&data[21..=28])?,
            price: Price::parse(&data[29..=36])?,
            time_in_force: TimeInForce::parse(data[37])?,
            display: Display::parse(data[38])?,
            order_ref_num: u64_from_be_bytes(&data[39..=46])?,
            capacity: Capacity::parse(data[47])?,
            intermarket_sweep_eligibility: match data[48] {
                b'Y' => true,
                b'N' => false,
            },
            cross_type: CrossType::parse(&data[49])?,
            order_state: OrderState::parse(&data[50])?,
            order_token: OrderToken::parse(&data[51..=64])?,
            optional_appendage: OptionalAppendage::parse(&data[65..])?
        })
    }

}
