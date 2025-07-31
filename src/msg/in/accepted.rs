
use serde::{ Deserialize, Serialize };
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAccepted {
    pub order_token: String,
    pub timestamp: u64,
    pub shares: u32,
    pub stock: String,
    pub price: u32,
    pub buy_sell_indicator: char,
}

impl OrderAccepted {

    pub(super) fn parse(data: &[u8]) -> Result<Self, String> {

        if data.len() < 39 {
            return Err("OrderAccepted: insufficient data".into());
        }

        Ok(OrderAccepted {
            order_token: String::from_utf8_lossy(&data[0..14]).trim_end().to_string(),
            timestamp: u64::from_be_bytes(data[14..22].try_into().unwrap()),
            shares: u32::from_be_bytes(data[22..26].try_into().unwrap()),
            stock: String::from_utf8_lossy(&data[26..34]).trim_end().to_string(),
            price: u32::from_be_bytes(data[34..38].try_into().unwrap()),
            buy_sell_indicator: data[38] as char,
        })
    }

}
