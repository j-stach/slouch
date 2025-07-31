
use serde::{ Deserialize, Serialize };
use super::encode_fixed_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceOrder {
    pub original_order_token: String, // 14 bytes
    pub new_order_token: String, // 14 bytes
    pub shares: u32,
    pub price: u32,
    pub time_in_force: u32,
}

impl ReplaceOrder {

    pub(super) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![b'U'];
        buf.extend(encode_fixed_str(&self.original_order_token, 14));
        buf.extend(encode_fixed_str(&self.new_order_token, 14));
        buf.extend(&self.shares.to_be_bytes());
        buf.extend(&self.price.to_be_bytes());
        buf.extend(&self.time_in_force.to_be_bytes());
        buf
    }

}
