
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


/// Cancel shares on an active order.
#[derive(Debug, Clone)]
pub struct CancelOrder {
    user_ref_num: UserRefNum,
    // This is the new intended order size. 
    // TODO: TBD: Does going over orig quantity do nothing?
    // This limits the maximum number of shares that can potentially 
    // be executed in total after the cancel is applied. 
    // Entering `0` will cancel any remaining open shares on this order.
    quantity: u32,
    optional_appendage: OptionalAppendage
}

impl CancelOrder {

    /// Create a new Cancel order. 
    ///
    /// `quantity` limits the maximum number of shares that can potentially 
    /// be executed in total after the cancel is applied. 
    /// Entering over 1,000,000 (maximum shares per order) results in an error.
    // TBD: Entering a value greater than the original quantity does nothing.
    /// Entering `0` will cancel all remaining open shares on this order.
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

    /// Gets the user reference number as a u32.
    pub fn user_ref_num(&self) -> u32 {
        self.user_ref_num.val()
    }

    /// Quantity of shares that will remain to be executed after canceling.
    pub fn quantity(&self) -> u32 {
        self.quantity
    }
    
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
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
} 


/// Cancel all active orders for a symbol.
#[derive(Debug, Clone)]
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
    ) -> Result<Self, BadElementError> {

        Ok(Self {
            user_ref_num,
            firm,
            symbol,
            optional_appendage: OptionalAppendage::new(),
        })
    }

    /// Gets the user reference number as a u32.
    pub fn user_ref_num(&self) -> u32 {
        self.user_ref_num.val()
    }

    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> &FirmId { &self.firm }
    
    /// Gets the symbol for which the orders will be canceled.
    pub fn symbol(&self) -> &StockSymbol { &self.symbol }
    
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
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
    
} 

