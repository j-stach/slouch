
use serde::{ Deserialize, Serialize };
use crate::types::OrderToken;

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceOrder {
    pub original_order_token: OrderToken,
    pub new_order_token: OrderToken,
    pub shares: u32,
    pub price: u32,
    pub time_in_force: u32,
}

impl ReplaceOrder {

    pub(super) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![b'U'];
        buf.extend(self.original_order_token.encode());
        buf.extend(self.new_order_token.encode());
        buf.extend(&self.shares.to_be_bytes());
        buf.extend(&self.price.to_be_bytes());
        buf.extend(&self.time_in_force.to_be_bytes());
        buf
    }

}
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceOrder {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: u32,
    /// Number of shares.
    quantity: u32,
    // TODO: Price type with decimal bounds
    price: u64,
    // TODO: Enum for Day, IOC, GTX, GTT, E
    time_in_force: char,
    // TODO: Enum for Visible, Hidden, Attributable
    display: char,
    // TODO: true -> Y, false -> N
    intermarket_sweep_eligibility: bool,
    /// Customer order identifier (ClOrdID). 
    /// Will not be checked for day-uniqueness for each OUCH account.
    // TODO: Modify OrderToken in form and purpose, still 14
    order_token: OrderToken,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl EnterOrder {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

