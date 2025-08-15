
use serde::{ Deserialize, Serialize };
use crate::{
    types::{ OrderToken, StockSymbol},
    error::OuchError,
    helper::{ u32_from_be_bytes, u64_from_be_bytes }
};


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderAccepted {
    pub order_token: OrderToken,
    pub timestamp: u64,
    pub shares: u32,
    pub stock_symbol: StockSymbol,
    pub price: u32,
    pub buy_sell_indicator: char,
}

impl OrderAccepted {

    pub(super) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        if data.len() < 39 {
            return Err(OuchError::Parse("OrderAccepted".to_string()))
        }

        let order_token = OrderToken::new(
            String::from_utf8_lossy(&data[0..14])
                .trim_end()
                .to_string()
        )?;

        let timestamp = u64_from_be_bytes(&data[14..22])?;
        let shares = u32_from_be_bytes(&data[22..26])?;

        let stock_symbol = StockSymbol::new(
            String::from_utf8_lossy(&data[26..34])
                .trim_end()
                .to_string()
        )?;

        let price = u32_from_be_bytes(&data[34..38])?;

        // TODO Enum
        let buy_sell_indicator = data[38] as char;

        Ok(OrderAccepted {
            order_token,
            timestamp,
            shares,
            stock_symbol,
            price,
            buy_sell_indicator,
        })
    }

}
