
use serde::{ Deserialize, Serialize };
use crate::{
    types::OrderToken,
    error::OuchError,
    helper::{ u64_from_be_bytes, u32_from_be_bytes }
};


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderExecuted {
    pub order_token: OrderToken,
    pub executed_shares: u32,
    pub execution_price: u32,
    pub match_number: u64,
}

impl OrderExecuted {

    pub(super) fn parse(data: &[u8]) -> Result<OrderExecuted, OuchError> {

        if data.len() < 30 {
            return Err(OuchError::Parse("OrderExecuted".to_string()))
        }

        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        let executed_shares = u32_from_be_bytes(&data[14..18])?;
        let execution_price = u32_from_be_bytes(&data[18..22])?;
        let match_number = u64_from_be_bytes(&data[22..30])?;

        Ok(OrderExecuted {
            order_token,
            executed_shares,
            execution_price,
            match_number
        })
    }

}

