
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

    /// Gets the user reference number as a u32.
    pub fn user_ref_num(&self) -> u32 {
        self.user_ref_num.val()
    }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Gets the symbol for which the orders will be placed.
    pub fn symbol(&self) -> &StockSymbol { &self.symbol }

    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> &Side { &self.side }

    pub fn price(&self) -> &Price { &self.price }

    pub fn time_in_force(&self) -> &TimeInForce { &self.time_in_force }

    pub fn display(&self) -> &Display { &self.display }

    pub fn capacity(&self) -> &Capacity { &self.capacity }

    pub fn intermarket_sweep_eligibility(&self) -> bool {
        self.intermarket_sweep_eligibility
    }

    pub fn cross_type(&self) -> &CrossType { &self.cross_type }

    pub fn order_token(&self) -> &OrderToken { &self.order_token }
    
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

