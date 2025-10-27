
use nsdq_util::Price;

use crate::{
    error::BadElementError,
    types::{
        OrderToken,
        UserRefNum,
        TimeInForce,
        Display
    },
    msg::options::{
        OptionalAppendage,
        TagValue
    },
};


/// Cancel and replace an existing order in a single message.
///
/// If the original order is no longer live or if the new UserRefNum is invalid,
/// the replacement will be silently ignored. 
///
/// If the original order is live but the details of the replace are invalid, 
/// the original order will be canceled but a new one will not be entered. 
/// In this case, the new UserRefNum will not be consumed and may be reused.
///
/// If the original order is live but the cannot be canceled 
/// (e.g., the existing Order is a cross order in the late period), 
/// there will be an OrderReject reponse and the order will not be replaced. 
/// The OrderReject consumes the new UserRefNum, so it may not be reused.
///
/// Replacing an order gives it a new timestamp/time priority on the book.
/// See the OUCH 5.0 specification for more information, including restrictions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceOrder {
    old_user_ref_num: UserRefNum,
    new_user_ref_num: UserRefNum,
    quantity: u32,
    price: Price<u64, 4>,
    time_in_force: TimeInForce,
    display: Display,
    intermarket_sweep_eligibility: bool,
    order_token: OrderToken,
    optional_appendage: OptionalAppendage
}

impl ReplaceOrder {
    
    /// Crate a new Replace order.
    ///
    /// The first UserRefNum is that of the existing order to cancel; 
    /// the second must be a valid new UserRefNum for the replacement order.
    ///
    /// If a `TagValue::UserRefIndex` option is used on the original order, 
    /// it must also be added to this request. (See `add_option` below.)
    pub fn new(
        old_user_ref_num: UserRefNum,
        new_user_ref_num: UserRefNum,
        quantity: u32,
        price: Price<u64, 4>,
        time_in_force: TimeInForce,
        display: Display,
        intermarket_sweep_eligibility: bool,
        order_token: OrderToken,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            old_user_ref_num,
            new_user_ref_num,
            quantity,
            price,
            time_in_force,
            display,
            intermarket_sweep_eligibility,
            order_token,
            optional_appendage: OptionalAppendage::new()
        })
    }

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        old_user_ref_num: UserRefNum,
        new_user_ref_num: UserRefNum,
        quantity: u32,
        price: Price<u64, 4>,
        time_in_force: TimeInForce,
        display: Display,
        intermarket_sweep_eligibility: bool,
        order_token: OrderToken,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(
            old_user_ref_num,
            new_user_ref_num,
            quantity,
            price,
            time_in_force,
            display,
            intermarket_sweep_eligibility,
            order_token
        ).expect("Quantity is acceptable value")
    }

    /// User reference number of the order to be replaced.
    pub fn old_user_ref_num(&self) -> UserRefNum { self.old_user_ref_num }

    /// User reference number for the new order.
    pub fn new_user_ref_num(&self) -> UserRefNum { self.new_user_ref_num }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Price at which the order will be placed.
    pub fn price(&self) -> Price<u64, 4> { self.price }

    /// Time block where the order is active (e.g., Day).
    /// "Corresponds to TimeInForce (59) in Nasdaq FIX."
    pub fn time_in_force(&self) -> TimeInForce { self.time_in_force }

    /// Visibility options set for this order.
    pub fn display(&self) -> Display { self.display }

    /// Returns true if this order is an eligible Intermarket Sweep Order.
    pub fn intermarket_sweep_eligibility(&self) -> bool {
        self.intermarket_sweep_eligibility
    }

    /// User-defined token (CIOrdId) that is set for this order. 
    /// Can be used to differentiate strategies, etc.
    pub fn order_token(&self) -> OrderToken { self.order_token }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
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
    /// NOTE: If a `UserRefIndex` option is used on the original order, 
    /// it MUST also be added here.
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
        bytes.extend(self.old_user_ref_num.encode());
        bytes.extend(self.new_user_ref_num.encode());
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

    /// Encode the request to a protocol-compliant byte array.
    pub fn to_bytes(&self) -> Vec<u8> { self. encode() }
} 

