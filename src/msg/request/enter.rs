
use nsdq_util::{
    StockSymbol, 
    Price,
};

use crate::error::BadElementError;

use crate::types::{ 
    UserRefNum,
    Side,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnterOrder {
    user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
    symbol: StockSymbol,
    price: Price<u64, 4>,
    time_in_force: TimeInForce,
    display: Display,
    capacity: Capacity,
    intermarket_sweep_eligibility: bool,
    cross_type: CrossType,
    order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl EnterOrder {

    /// Create a new Enter order.
    /// For `quantity`, entering over 1,000,000 (maximum shares per order) 
    /// results in an error.
    pub fn new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
        symbol: StockSymbol,
        price: Price<u64, 4>,
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

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
        symbol: StockSymbol,
        price: Price<u64, 4>,
        time_in_force: TimeInForce,
        display: Display,
        capacity: Capacity,
        intermarket_sweep_eligibility: bool,
        cross_type: CrossType,
        order_token: OrderToken,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(
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
            order_token
        ).expect("Quantity is acceptable value")
    }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Symbol for which the orders will be placed.
    pub fn symbol(&self) -> StockSymbol { self.symbol }

    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> Side { self.side }

    /// Price at which the order will be placed.
    pub fn price(&self) -> Price<u64, 4> { self.price }

    /// Time block where the order is active (e.g., Day).
    /// "Corresponds to TimeInForce (59) in Nasdaq FIX."
    pub fn time_in_force(&self) -> TimeInForce { self.time_in_force }

    /// Visibility options set for this order.
    pub fn display(&self) -> Display { self.display }

    /// Trading capacity with which this order will be placed.
    /// (e.g. Agency, Principal)
    pub fn capacity(&self) -> Capacity { self.capacity }

    /// Returns true if this order is an eligible Intermarket Sweep Order.
    pub fn intermarket_sweep_eligibility(&self) -> bool {
        self.intermarket_sweep_eligibility
    }

    /// The market event for when this order is to be executed.
    pub fn cross_type(&self) -> CrossType { self.cross_type }

    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_token(&self) -> OrderToken { self.order_token }
    
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
            UserRefIndex(..) 
                => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "EnterOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'O'); // Type identifier for Enter Order Request
        bytes.extend(self.user_ref_num.encode());
        bytes.push(self.side.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.symbol.encode());
        bytes.extend(self.price.encode());
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
    
    /// Encode the request to a protocol-compliant byte array.
    pub fn to_bytes(&self) -> Vec<u8> { self. encode() }
} 

