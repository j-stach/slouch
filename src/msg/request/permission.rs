
use crate::error::BadElementError;

use crate::types::{
    UserRefNum,
    FirmId
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};


///
#[derive(Debug, Clone)]
pub struct DisableOrderEntry {
    user_ref_num: UserRefNum,
    firm: FirmId,
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntry {

    pub fn new(
        user_ref_num: UserRefNum,
        firm: FirmId,
    ) -> Self {
        
        Self {
            user_ref_num,
            firm,
            optional_appendage: OptionalAppendage::new(),
        }
    }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }
    
    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> &FirmId { &self.firm }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'D');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
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
                    "DisableOrderEntry".to_string()
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

///
#[derive(Debug, Clone)]
pub struct EnableOrderEntry {
    user_ref_num: UserRefNum,
    firm: FirmId,
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntry {

    pub fn new(
        user_ref_num: UserRefNum,
        firm: FirmId,
    ) -> Self {
        
        Self {
            user_ref_num,
            firm,
            optional_appendage: OptionalAppendage::new(),
        }
    }

    /// Gets the user reference number.
    pub fn user_ref_num(&self) -> UserRefNum { self.user_ref_num }
    
    /// Gets the ID for the firm for whom the orders will be canceled.
    pub fn firm(&self) -> &FirmId { &self.firm }
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'E');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
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
                    "EnableOrderEntry".to_string()
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

