
use serde::{ Deserialize, Serialize };
use crate::types::{
    UserRefNum,
    FirmId,
    StockSymbol,
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};


/// Cancel an active order.
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    pub user_ref_num: UserRefNum,
    pub quantity: u32,
    optional_appendage: OptionalAppendage
}

impl CancelOrder {
    
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
                return BadElementError::InvalidOption(
                    "CancelOrder".to_string()
                )
            },
        }

        Ok(self.optional_appendage.add(option))
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
} 


/// Cancel all active orders for a Symbol.
#[derive(Debug, Serialize, Deserialize)]
pub struct MassCancel {
    pub user_ref_num: UserRefNum,
    pub firm: FirmId,
    pub symbol: StockSymbol,
    optional_appendage: OptionalAppendage
}

impl MassCancel {
    
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
            GroupId(..) ||
            Side(..) ||
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return BadElementError::InvalidOption(
                    "MassCancel".to_string()
                )
            },
        }

        Ok(self.optional_appendage.add(option))
    }
    
    /// Get read-only access to the OptionalAppendage.
    pub fn options(&self) -> &OptionalAppendage {
        &self.optional_appendage
    }
    
} 

