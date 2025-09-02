
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
    // TODO: Enum for Buy, Sell, Sell short, Sell short exempt
    side: Side,
    /// Number of shares.
    quantity: u32,
    // Still 8?
    symbol: StockSymbol,
    // TODO: Price type with decimal bounds
    price: Price,
    // TODO: Enum for Day, IOC, GTX, GTT, E
    time_in_force: TimeInForce,
    // TODO: Enum for Visible, Hidden, Attributable
    display: Display,
    // TODO: Agency, Principle, Riskless
    capacity: Capacity,
    // TODO: true -> Y, false -> N
    intermarket_sweep_eligibility: bool,
    // TODO: Needs enum, see the PDF
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

