
use serde::{ Deserialize, Serialize };
use super::encode_fixed_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    pub order_token: String, // 14 bytes
    pub cancel_shares: u32,
}

impl CancelOrder {

    pub(super) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![b'X'];
        buf.extend(encode_fixed_str(&self.order_token, 14));
        buf.extend(self.cancel_shares.to_be_bytes());
        buf
    }


}
