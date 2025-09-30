
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

use crate::msg::options::{ 
    OptionalAppendage,
    TagValue
};


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
    pub intermarket_sweep_eligibility: bool,
    pub cross_type: CrossType,
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl EnterOrder {

    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
    /// - Firm
    /// - MinQty
    /// - CustomerType
    /// - MaxFloor
    /// - PriceType
    /// - PegOffset
    /// - DiscretionPrice
    /// - DiscretionPriceType
    /// - DiscretionPegOffset
    /// - PostOnly
    /// - RandomReserves
    /// - ExpireTime
    /// - TradeNow
    /// - HandleInst
    /// - GroupId
    /// - SharesLocated
    /// - LocateBroker
    /// - UserRefIndex
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        use TagValue::*;
        match option {
            Firm(..) ||
            MinQty(..) ||
            CustomerType(..) ||
            MaxFloor(..) ||
            PriceType(..) ||
            PegOffset(..) ||
            DiscretionPrice(..) ||
            DiscretionPriceType(..) ||
            DiscretionPegOffset(..) ||
            PostOnly(..) ||
            RandomReserves(..) ||
            ExpireTime(..) ||
            TradeNow(..) ||
            HandleInst(..) ||
            GroupId(..) ||
            SharesLocated(..) ||
            LocateBroker(..) ||
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return BadElementError::InvalidOption(
                    "EnterOrder".to_string()
                )
            },
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
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
} 

