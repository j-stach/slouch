
use serde::{ Deserialize, Serialize };
use crate::types::OrderToken;

/*
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
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: u32,
    /// Number of shares.
    quantity: u32,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl CancelOrder {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct MassCancel {
    /// Must be day-unique and strictly increasing for each OUCH account.
    // TODO: Client should track and increment this
    // TODO: Data type
    user_ref_num: u32,
    // TODO: All caps 4 char string
    firm: FirmId,
    // Still 8?
    symbol: StockSymbol,
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl MassCancel {
    
    pub(super) fn encode(&self) -> Vec<u8> {
        todo!{}
    }
} 

