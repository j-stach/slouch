
use serde::{ Deserialize, Serialize };

use crate::types::{ 
    UserRefNum,
    Side,
    StockSymbol, 
    Price,
    TimeInForce,
    Display,
    Capacity,
    CrossType,
    OrderToken
};
use super::options::OptionalAppendage;


/// Enter a new order.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnterOrder {
    /// Must be day-unique and strictly increasing for each OUCH account.
    pub user_ref_num: UserRefNum,
    pub side: Side,
    pub quantity: u32,
    pub symbol: StockSymbol,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub capacity: Capacity,
    // true -> Y, false -> N
    pub intermarket_sweep_eligibility: bool,
    pub cross_type: CrossType,
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl EnterOrder {

    /// Add an optional field to the optional appendage.
    pub fn add_option(
        &mut self, 
        option: OptionValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable OptionValues for EnterOrder.
        match option {
            Route ||
            BboWeightIndicator ||
            DisplayQuantity ||
            DisplayPrice => {
                return BadElementError::InvalidOption(
                    "EnterOrder".to_string()
                )
            },

            _ => {/* Do nothing */},
        }

        Ok(self.optional_appendage.add(option))
    }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'O'); // Type identifier for Enter Order Request
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.side.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.stock_symbol.encode());
        bytes.extend(self.time_in_force.encode());
        bytes.extend(self.display.encode());
        bytes.extend(self.capacity.encode());
        bytes.push(match self.intermarket_sweep_eligibility {
            true => b'Y',
            false => b'N',
        });
        bytes.extend(self.cross_type.encode());
        bytes.extend(self.order_token.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
} 

