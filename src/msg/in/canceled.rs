
use serde::{ Deserialize, Serialize };
use crate::token::OrderToken;


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCanceled {
    pub order_token: OrderToken,
    pub canceled_shares: u32,
}

impl OrderCanceled {

    pub(super) fn parse(data: &[u8]) -> Result<OrderCanceled, String> {

        if data.len() < 18 {
            return Err("OrderCanceled: insufficient data".into());
        }

        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14]).trim_end().to_string()
        )?;

        let canceled_shares = u32::from_be_bytes(data[14..18].try_into().unwrap());

        Ok(OrderCanceled {
            order_token,
            canceled_shares,
        })
    }

}

