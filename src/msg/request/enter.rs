
use serde::{ Deserialize, Serialize };

use crate::types::{ 
    UserRefNum,
    Side,
    StockSymbol, 
    Price,
    TimeInForce,
    Display,
    Capacity,
    CrossType,
    OrderToken
};
use super::options::OptionalAppendage;


#[derive(Debug, Serialize, Deserialize)]
pub struct EnterOrder {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
    // Still 8?
    symbol: StockSymbol,
    // TODO: Price type with decimal bounds
    price: Price,
    time_in_force: TimeInForce,
    display: Display,
    capacity: Capacity,
    // TODO: true -> Y, false -> N
    intermarket_sweep_eligibility: bool,
    cross_type: CrossType,
    /// Customer order identifier (ClOrdID). 
    /// Will not be checked for day-uniqueness for each OUCH account.
    // TODO: Modify OrderToken in form and purpose, still 14
    order_token: OrderToken,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl EnterOrder {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        // TODO: Encode each field and push to byte string in order

        bytes
    }
} 

