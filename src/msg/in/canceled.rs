
use serde::{ Deserialize, Serialize };
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCanceled {
    pub order_token: String,
    pub canceled_shares: u32,
}

impl OrderCanceled {

    pub(super) fn parse(data: &[u8]) -> Result<OrderCanceled, String> {

        if data.len() < 18 {
            return Err("OrderCanceled: insufficient data".into());
        }

        Ok(OrderCanceled {
            order_token: String::from_utf8_lossy(&data[0..14]).trim_end().to_string(),
            canceled_shares: u32::from_be_bytes(data[14..18].try_into().unwrap()),
        })
    }

}

