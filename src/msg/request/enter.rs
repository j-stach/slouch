
use crate::error::BadElementError;

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
/// For `quantity`, entering over 1,000,000 (maximum shares per order) 
/// results in an error.
#[derive(Debug, Clone)]
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

    ///
    pub fn new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
        symbol: StockSymbol,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        capacity: Capacity,
        intermarket_sweep_eligibility: bool,
        cross_type: CrossType,
        order_token: OrderToken,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            side,
            quantity,
            symbol,
            price,
            time_in_force,
            display,
            capacity,
            intermarket_sweep_eligibility,
            cross_type,
            order_token,
            optional_appendage: OptionalAppendage::new()
        })
    }

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
            Firm(..) |
            MinQty(..) |
            CustomerType(..) |
            MaxFloor(..) |
            PriceType(..) |
            PegOffset(..) |
            DiscretionPrice(..) |
            DiscretionPriceType(..) |
            DiscretionPegOffset(..) |
            PostOnly(..) |
            RandomReserves(..) |
            ExpireTime(..) |
            TradeNow(..) |
            HandleInst(..) |
            GroupId(..) |
            SharesLocated(..) |
            LocateBroker(..) |
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "EnterOrder".to_string()
                ))
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
        bytes.extend(self.symbol.encode());
        bytes.push(self.time_in_force.encode());
        bytes.push(self.display.encode());
        bytes.push(self.capacity.encode());
        bytes.push(match self.intermarket_sweep_eligibility {
            true => b'Y',
            false => b'N',
        });
        bytes.push(self.cross_type.encode());
        bytes.extend(self.order_token.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
} 

