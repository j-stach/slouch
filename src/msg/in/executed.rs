
use serde::{ Deserialize, Serialize };
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderExecuted {
    pub order_token: String,
    pub executed_shares: u32,
    pub execution_price: u32,
    pub match_number: u64,
}

impl OrderExecuted {

    pub(super) fn parse(data: &[u8]) -> Result<OrderExecuted, String> {
        if data.len() < 30 {
            return Err("OrderExecuted: insufficient data".into());
        }
        Ok(OrderExecuted {
            order_token: String::from_utf8_lossy(&data[0..14]).trim_end().to_string(),
            executed_shares: u32::from_be_bytes(data[14..18].try_into().unwrap()),
            execution_price: u32::from_be_bytes(data[18..22].try_into().unwrap()),
            match_number: u64::from_be_bytes(data[22..30].try_into().unwrap()),
        })
    }

}

