
mod enter;
mod cancel;
mod replace;

pub use self::{ 
    enter::EnterOrder, 
    cancel::CancelOrder, 
    replace::ReplaceOrder 
};


use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum OuchOut {
    EnterOrder(EnterOrder),
    CancelOrder(CancelOrder),
    ReplaceOrder(ReplaceOrder),
}

impl OuchOut {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            OuchOut::EnterOrder(msg) => msg.encode(),
            OuchOut::CancelOrder(msg) => msg.encode(),
            OuchOut::ReplaceOrder(msg) => msg.encode(),
        }
    }
}

