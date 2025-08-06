
use serde::{ Deserialize, Serialize };

use crate::helper::encode_fixed_str;
use crate::token::OrderToken;


#[derive(Debug, Serialize, Deserialize)]
pub struct EnterOrder {
    pub order_token: OrderToken,
    pub buy_sell_indicator: char,
    pub shares: u32,
    pub stock: String, // 8 bytes ASCII
    pub price: u32,
    pub time_in_force: u32,
    pub firm: String, // 4 bytes
    pub display: char,
    pub capacity: char,
    pub intermarket_sweep_eligibility: char,
    pub minimum_quantity: u32,
    pub cross_type: char,
    pub customer_type: char,
}

impl EnterOrder {

    pub(super) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![b'O'];
        buf.extend(self.order_token.encode());
        buf.push(self.buy_sell_indicator as u8);
        buf.extend(&self.shares.to_be_bytes());
        buf.extend(encode_fixed_str(&self.stock, 8));
        buf.extend(&self.price.to_be_bytes());
        buf.extend(&self.time_in_force.to_be_bytes());
        buf.extend(encode_fixed_str(&self.firm, 4));
        buf.push(self.display as u8);
        buf.push(self.capacity as u8);
        buf.push(self.intermarket_sweep_eligibility as u8);
        buf.extend(&self.minimum_quantity.to_be_bytes());
        buf.push(self.cross_type as u8);
        buf.push(self.customer_type as u8);
        buf
    }

}

