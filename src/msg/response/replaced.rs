
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

use crate::msg::options::*;


/// Acknowledges the receipt and acceptance of a valid ReplaceOrder request. 
/// The data fields from the request are echoed back in this message. 
/// NOTE: Accepted values may differ from the entered values for some fields.
///
// TBD: You will receive one and only one of these two for each replacement.
/// `OrderState::Dead` indicates that a replace was automatically canceled,
/// and no further messages will be received for the order. 
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
    
    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Gets the previous user reference number.
    pub fn old_user_ref_num(&self) -> UserRefNum {
        self.old_user_ref_num
    }

    /// Gets the new user reference number.
    pub fn new_user_ref_num(&self) -> UserRefNum {
        self.new_user_ref_num
    }

    /// Quantity of shares left exposed when the replacement completed.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Gets the symbol for which the orders will be placed.
    pub fn symbol(&self) -> &StockSymbol { &self.symbol }

    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> Side { self.side }

    /// The accepted price of the replacement. 
    /// NOTE: The accepted price could potentially be different 
    /// than the entered price if the order was re-priced by NASDAQ on entry. 
    /// The accepted price will always be better than or equal to the entered.
    pub fn price(&self) -> Price { self.price }

    /// The accepted Time in Force of the replacement. 
    /// NOTE: The accepted Time in Force may potentially be different 
    /// than the entered Time in Force. 
    /// The accepted Time in Force will always be equal to or shorter in scope 
    /// than the entered Time in Force.
    pub fn time_in_force(&self) -> TimeInForce { self.time_in_force }

    /// Visibility options set for this order.
    pub fn display(&self) -> Display { self.display }

    /// Day-unique Order Reference Number assigned by NASDAQ to this order.
    pub fn order_ref_num(&self) -> u64 { self.order_ref_num }

    /// Order state (Live or Dead)
    pub fn order_state(&self) -> OrderState { self.order_state }

    /// Trading capacity with which this order will be placed.
    /// (e.g. Agency, Principal)
    pub fn capacity(&self) -> Capacity { self.capacity }

    /// Returns true if this order is an eligible Intermarket Sweep Order.
    pub fn intermarket_sweep_eligibility(&self) -> bool {
        self.intermarket_sweep_eligibility
    }

    /// The market event for when this order is to be executed.
    pub fn cross_type(&self) -> CrossType { self.cross_type }

    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_token(&self) -> &OrderToken { &self.order_token }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}
