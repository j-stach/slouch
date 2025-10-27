
use nom::number::streaming::{ be_u32, be_u64 };

use nsdq_util::{
    NaiveTime,
    parse_ouch_time_bold,
    parse_bool,
    Price,
    StockSymbol,
};

use crate::{
    error::{ OuchError, BadElementError },
    msg::options::*,
    types::{ 
        UserRefNum,
        Side,
        TimeInForce,
        Display,
        Capacity,
        CrossType,
        OrderState,
        OrderToken
    },
};


/// Acknowledges the receipt and acceptance of a valid EnterOrder request. 
/// The data fields from the Enter request are echoed back in this message. 
/// NOTE: Accepted values may differ from the entered values for some fields.
///
/// OrderAccepted normally comes before any OrderExecuted or OrderCanceled. 
/// However, when the Order State field of an Accepted Message is Dead (“D”), 
/// no additional messages will be received for that order, 
/// as Order Dead means that the order was accepted and automatically canceled.
#[derive(Debug, Clone)]
pub struct OrderAccepted {
    timestamp: NaiveTime,
    user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
    symbol: StockSymbol,
    price: Price<u64, 4>,
    time_in_force: TimeInForce,
    display: Display,
    order_ref_num: u64,
    capacity: Capacity,
    intermarket_sweep: bool,
    cross_type: CrossType,
    order_state: OrderState,
    order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl OrderAccepted {

    // Data contains package without type tag, 
    // so all offsets should be one less than those in the official spec.
    pub(super) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, timestamp) = parse_ouch_time_bold(input)?;
        let (input, user_ref_num) = UserRefNum::parse()?;
        let (input, side) = Side::parse(input)?;
        let (input, quantity) = be_u32(input)?;
        let (input, symbol) = StockSymbol::parse(input)?;
        let (input, price) = Price<u64, 4>::parse(input)?;
        let (input, time_in_force) = TimeInForce::parse(input)?;
        let (input, display) = Display::parse(input)?;
        let (input, order_ref_num) = be_u64(input)?;
        let (input, capacity) = Capacity::parse(input)?;
        let (input, intermarket_sweep) = parse_bool(input)?;
        let (input, cross_type) = CrossType::parse(input)?;
        let (input, order_state) = OrderState::parse(input)?;
        let (input, order_token) = OrderToken::parse(input)?;
        let (input, optional_appendage) = OptionalAppendage::parse(input)?;


        Ok((input, Self {
            timestamp, 
            user_ref_num, 
            side, 
            quantity, 
            symbol,
            price, 
            time_in_force, 
            display, 
            order_ref_num,
            capacity, 
            intermarket_sweep, 
            cross_type,
            order_state, 
            order_token, 
            optional_appendage 
        }))
    }

    /// Time this message was generated.
    pub fn timestamp(&self) -> NaiveTime { self.timestamp }
    
    /// Refers to the order which was accepted.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Gets the symbol for which the orders will be placed.
    pub fn symbol(&self) -> StockSymbol { self.symbol }

    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> Side { self.side }

    /// The accepted price of the order. 
    /// NOTE: The accepted price could potentially be different 
    /// than the entered price if the order was re-priced by NASDAQ on entry. 
    /// The accepted price will always be better than or equal to the entered.
    pub fn price(&self) -> Price<u64, 4> { self.price }

    /// The accepted Time in Force of the order. 
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
    pub fn intermarket_sweep(&self) -> bool {
        self.intermarket_sweep
    }

    /// The market event for when this order is to be executed.
    pub fn cross_type(&self) -> CrossType { self.cross_type }

    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_token(&self) -> OrderToken { self.order_token }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }

}

