
use serde::{ Deserialize, Serialize };
use crate::types::OrderToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    pub order_token: OrderToken,
    pub cancel_shares: u32,
}

impl CancelOrder {

    pub(super) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![b'X'];
        buf.extend(self.order_token.encode());
        buf.extend(self.cancel_shares.to_be_bytes());
        buf
    }


}
