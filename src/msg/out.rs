
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

// Helper function
pub(self) fn encode_fixed_str(s: &str, len: usize) -> Vec<u8> {
    let mut buf = vec![b' '; len];
    let bytes = s.as_bytes();
    let copy_len = usize::min(len, bytes.len());
    buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
    buf
}

