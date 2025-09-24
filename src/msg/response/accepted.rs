
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

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 63 {
            return Err(OuchError::Parse("OrderAccepted".to_string()))
        }

        Ok(Self {
            timestamp: {
                let ts = u64_from_be_bytes(&data[0..=7])?;
                nanosec_from_midnight(ts)
            },
            user_ref_num: UserRefNum::parse(&data[8..=11])?,
            side: Side::parse(data[12])?, 
            quantity: u32_from_be_bytes(&data[13..=16])?, 
            symbol: StockSymbol::parse(&data[17..=24])?,
            price: Price::parse(&data[25..=32])?,
            time_in_force: TimeInForce::parse(data[33])?,
            display: Display::parse(data[34])?,
            order_ref_num: u64_from_be_bytes(&data[35..=42])?,
            capacity: Capacity::parse(data[43])?,
            intermarket_sweep_eligibility: match data[44] {
                b'Y' => true,
                b'N' => false,
            },
            cross_type: CrossType::parse(&data[45])?,
            order_state: OrderState::parse(&data[46])?,
            order_token: OrderToken::parse(&data[47..=60])?,
            optional_appendage: OptionalAppendage::parse(&data[61..])?
        })
    }

}

