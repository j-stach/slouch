
use crate::error::BadElementError;

use crate::types::{
    UserRefNum,
    Side
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};

///
#[derive(Debug, Clone)]
pub struct ModifyOrder {
    user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
    optional_appendage: OptionalAppendage
}

impl ModifyOrder {

    ///
    pub fn new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            side,
            quantity,
            optional_appendage: OptionalAppendage::new(),
        })
    }
    
    /// Gets the user reference number as a u32.
    pub fn user_ref_num(&self) -> u32 {
        self.user_ref_num.val()
    }

    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> &Side { &self.side }

    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'X');
        bytes.extend(self.side.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }

    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
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
            SharesLocated(..) |
            LocateBroker(..) |
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "ModifyOrder".to_string()
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

