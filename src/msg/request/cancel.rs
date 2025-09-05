
use serde::{ Deserialize, Serialize };
use crate::types::{
    UserRefNum,
    FirmId,
    StockSymbol,
};

use crate::msg::options::OptionalAppendage;


/// Cancel an active order.
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    pub user_ref_num: UserRefNum,
    pub quantity: u32,
    optional_appendage: OptionalAppendage
}

impl CancelOrder {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'X');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
} 


/// Cancel all active orders for a Symbol.
#[derive(Debug, Serialize, Deserialize)]
pub struct MassCancel {
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
    pub symbol: StockSymbol,
    optional_appendage: OptionalAppendage
}

impl MassCancel {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'C');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
        bytes.extend(self.symbol.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
} 

