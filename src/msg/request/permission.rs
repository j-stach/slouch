
use crate::types::{
    UserRefNum,
    Firm
};

use crate::msg::options::{
    OptionalAppendage,
    TagValue
};


///
#[derive(Debug, Clone)]
pub struct DisableOrderEntry {
    pub user_ref_num: UserRefNum,
    pub firm: Firm,
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntry {
    
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
                return BadElementError::InvalidOption(
                    "DisableOrderEntry".to_string()
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

///
#[derive(Debug, Clone)]
pub struct EnableOrderEntry {
    pub user_ref_num: UserRefNum,
    pub firm: Firm,
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntry {
    
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
                return BadElementError::InvalidOption(
                    "EnableOrderEntry".to_string()
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

