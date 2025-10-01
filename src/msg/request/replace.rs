
use crate::error::BadElementError;

use crate::types::{
    OrderToken,
    UserRefNum,
    Price,
    TimeInForce,
    Display
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};


///
/// If a `UserRefIndex` option is used on the original order, 
/// it must also be added here.
#[derive(Debug, Clone)]
pub struct ReplaceOrder {
    pub user_ref_num: UserRefNum,
    pub quantity: u32,
    pub price: Price,
    pub time_in_force: TimeInForce,
    pub display: Display,
    pub intermarket_sweep_eligibility: bool,
    pub order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl ReplaceOrder {
    
    ///
    pub fn new(
        user_ref_num: UserRefNum,
        quantity: u32,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        intermarket_sweep_eligibility: bool,
        order_token: OrderToken,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            quantity,
            price,
            time_in_force,
            display,
            intermarket_sweep_eligibility,
            order_token,
            optional_appendage: OptionalAppendage::new()
        })
    }

    /// Add an optional field to the optional appendage.
    /// The majority of fields from the Enter Order Message are supported 
    /// in this message, except for `Firm` and `GroupId`, which are inherited
    /// from the original order. 
    /// Per spec, `CustomerType` is also not accepted, 
    /// although `Side` may be optionally supplied.
    /// Available options for this message type are:
    /// - MinQty
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
    /// - SharesLocated
    /// - LocateBroker
    /// - UserRefIndex
    /// - Side
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        use TagValue::*;
        match option {
            MinQty(..) |
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
            Side(..) |
            SharesLocated(..) |
            LocateBroker(..) |
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "ReplaceOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'U');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.price.encode());
        bytes.push(self.time_in_force.encode());
        bytes.push(self.display.encode());
        bytes.push(match self.intermarket_sweep_eligibility {
            true => b'Y',
            false => b'N',
        });
        bytes.extend(self.order_token.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
} 

