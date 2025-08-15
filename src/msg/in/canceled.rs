
use serde::{ Deserialize, Serialize };
use crate::{
    types::OrderToken,
    error::OuchError,
    helper::u32_from_be_bytes
};


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCanceled {
    pub order_token: OrderToken,
    pub canceled_shares: u32,
}

impl OrderCanceled {

    pub(super) fn parse(data: &[u8]) -> Result<OrderCanceled, OuchError> {

        if data.len() < 18 {
            return Err(OuchError::Parse("OrderCanceled".to_string()))
        }

        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        let canceled_shares = u32_from_be_bytes(&data[14..18])?;

        Ok(OrderCanceled {
            order_token,
            canceled_shares,
        })
    }

}

