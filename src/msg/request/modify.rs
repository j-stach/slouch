
use crate::error::BadElementError;

use crate::types::{
    UserRefNum,
    Side
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};

/// Modify values for an existing order, without affecting priority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifyOrder {
    user_ref_num: UserRefNum,
    side: Side,
    quantity: u32,
    optional_appendage: OptionalAppendage
}

impl ModifyOrder {

    /// Create a new Modify request.
    ///
    /// `user_ref_num` refers to the order to be modified.
    ///
    /// For `quantity`, entering over 1,000,000 (maximum shares per order) 
    /// results in an error.
    ///
    /// The following `side` modifications are allowed:
    /// Sell <-> SellShortExempt
    /// Sell <-> SellShort
    /// SellShortExempt <-> SellShort
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

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(user_ref_num, side, quantity)
            .expect("Quantity is acceptable value")
    }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }
    
    /// Quantity of shares to be ordered.
    pub fn quantity(&self) -> u32 { self.quantity }
    
    /// Market side (Buy, Sell, etc.)
    pub fn side(&self) -> Side { self.side }

    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
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

    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'M');
        bytes.extend(self.user_ref_num.encode());
        bytes.push(self.side.encode());
        bytes.extend(self.quantity.to_be_bytes());
        bytes.extend(self.optional_appendage.encode_nothing_if_empty());

        bytes
    }
    
    /// Encode the request to a protocol-compliant byte array.
    pub fn to_bytes(&self) -> Vec<u8> { self. encode() }
} 

