
use serde::{ Deserialize, Serialize };
use crate::{
    types::OrderToken,
    error::OuchError
};


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRejected {
    pub order_token: OrderToken,
    pub reason: char,
}

impl OrderRejected {

    pub(super) fn parse(data: &[u8]) -> Result<OrderRejected, OuchError> {

        if data.len() < 15 {
            return Err(OuchError::Parse("OrderRejected".to_string()))
        }
        
        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        // TODO Enum
        let reason = data[14] as char;

        Ok(OrderRejected {
            order_token,
            reason
        })
    }

}
