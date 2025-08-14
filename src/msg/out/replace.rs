
use serde::{ Deserialize, Serialize };
use crate::types::OrderToken;

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
