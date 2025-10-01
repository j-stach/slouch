
use chrono::NaiveTime;

use crate::{
    error::{ OuchError, BadElementError },
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
    OrderToken
};

use crate::msg::options::OptionalAppendage;


/// 
#[derive(Debug, Clone)]
pub struct OrderReplaced {
    timestamp: NaiveTime,
    old_user_ref_num: UserRefNum,
    new_user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
    symbol: StockSymbol,
    price: Price,
    time_in_force: TimeInForce,
    display: Display,
    order_ref_num: u64,
    capacity: Capacity,
    intermarket_sweep_eligibility: bool,
    cross_type: CrossType,
    order_state: OrderState,
    order_token: OrderToken,
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

                _ => return Err(BadElementError::InvalidEnum(
                    (data[44] as char).to_string(), 
                    "IntermarketSweepEligibility".to_string()
                ).into())
            },
            cross_type: CrossType::parse(data[49])?,
            order_state: OrderState::parse(data[50])?,
            order_token: OrderToken::parse(&data[51..=64])?,
            optional_appendage: OptionalAppendage::parse(&data[65..])?
        })
    }
    
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Gets the previous user reference number.
    pub fn old_user_ref_num(&self) -> UserRefNum {
        self.old_user_ref_num
    }

    /// Gets the new user reference number.
    pub fn new_user_ref_num(&self) -> UserRefNum {
        self.new_user_ref_num
    }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Gets the symbol for which the orders will be placed.
    pub fn symbol(&self) -> &StockSymbol { &self.symbol }

    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> Side { self.side }

    pub fn price(&self) -> Price { self.price }

    pub fn time_in_force(&self) -> TimeInForce { self.time_in_force }

    pub fn display(&self) -> Display { self.display }

    pub fn order_ref_num(&self) -> u64 { self.order_ref_num }

    pub fn order_state(&self) -> OrderState { self.order_state }

    pub fn capacity(&self) -> Capacity { self.capacity }

    pub fn intermarket_sweep_eligibility(&self) -> bool {
        self.intermarket_sweep_eligibility
    }

    pub fn cross_type(&self) -> CrossType { self.cross_type }

    pub fn order_token(&self) -> &OrderToken { &self.order_token }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }

}
