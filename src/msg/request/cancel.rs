
use crate::error::BadElementError;

use crate::types::{
    UserRefNum,
    FirmId,
    StockSymbol,
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};


/// Cancel or reduce shares on an existing order.
///
/// Duplicate cancel requests for the same UserRefNum will be ignored by OUCH.
/// Canceling an order after its execution will be silently ignored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelOrder {
    user_ref_num: UserRefNum,
    // TODO: TBD: Does going over orig quantity do nothing?
    quantity: u32,
    optional_appendage: OptionalAppendage
}

impl CancelOrder {

    /// Create a new Cancel order. 
    ///
    /// `user_ref_num` refers to the order to be canceled.
    ///
    /// `quantity` limits the maximum number of shares that remain to be 
    /// executed after the (partial) cancel is applied. 
    /// Entering over 1,000,000 (maximum shares per order) results in an error.
    // TBD: Entering a value greater than the original quantity does nothing.
    /// Entering `0` will cancel all remaining open shares on the order.
    pub fn new(
        user_ref_num: UserRefNum,
        quantity: u32,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            quantity,
            optional_appendage: OptionalAppendage::new(),
        })
    }

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        user_ref_num: UserRefNum,
        quantity: u32,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(user_ref_num, quantity)
            .expect("Quantity is acceptable value")
    }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Quantity of shares that will remain to be executed after canceling.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'X');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }

    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
    /// - UserRefIndex
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        match option {
            TagValue::UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "CancelOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }
} 


/// Cancel all active orders for a symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MassCancel {
    user_ref_num: UserRefNum,
    firm: FirmId,
    symbol: StockSymbol,
    optional_appendage: OptionalAppendage
}

impl MassCancel {

    /// Create a new Mass Cancel order. 
    pub fn new(
        user_ref_num: UserRefNum,
        firm: FirmId,
        symbol: StockSymbol,
    ) -> Self {

        Self {
            user_ref_num,
            firm,
            symbol,
            optional_appendage: OptionalAppendage::new(),
        }
    }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }

    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> FirmId { self.firm }
    
    /// Gets the symbol for which the orders will be canceled.
    pub fn symbol(&self) -> StockSymbol { self.symbol }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'C');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
        bytes.extend(self.symbol.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }

    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
    /// - GroupId
    /// - UserRefIndex
    /// - Side
    /// For `GroupId`: if the value is set to 0, 
    /// all orders without a Group ID will be canceled.
    /// If the value is not specified, all orders, regardless
    /// of whether they have a Group ID or not, will be canceled.
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        use TagValue::*;
        match option {
            GroupId(..) |
            Side(..) |
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "MassCancel".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
    }
    
} 

