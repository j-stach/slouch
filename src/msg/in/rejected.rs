
use serde::{ Deserialize, Serialize };
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRejected {
    pub order_token: String,
    pub reason: char,
}

impl OrderRejected {

    pub(super) fn parse(data: &[u8]) -> Result<OrderRejected, String> {

        if data.len() < 15 {
            return Err("OrderRejected: insufficient data".into());
        }
        
        Ok(OrderRejected {
            order_token: String::from_utf8_lossy(&data[0..14]).trim_end().to_string(),
            reason: data[14] as char,
        })
    }

}
