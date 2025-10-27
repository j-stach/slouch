
use nsdq_util::Mpid;

use crate::{ 
    types::UserRefNum,
    error::BadElementError,
    msg::options::{
        OptionalAppendage,
        TagValue
    }
};


/// Prevent a firm from entering new orders on this account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisableOrderEntry {
    user_ref_num: UserRefNum,
    firm: Mpid,
    optional_appendage: OptionalAppendage
}

impl DisableOrderEntry {

    /// Create a new Disable Entry request.
    pub fn new(
        user_ref_num: UserRefNum,
        firm: Mpid,
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
    pub fn firm(&self) -> Mpid { self.firm }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
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
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'D');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }
    
    /// Encode the request to a protocol-compliant byte array.
    pub fn to_bytes(&self) -> Vec<u8> { self. encode() }
} 

/// Allow a firm to enter new orders on this account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnableOrderEntry {
    user_ref_num: UserRefNum,
    firm: Mpid,
    optional_appendage: OptionalAppendage
}

impl EnableOrderEntry {

    /// Create a new Enable Entry request.
    pub fn new(
        user_ref_num: UserRefNum,
        firm: Mpid,
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
    pub fn firm(&self) -> Mpid { self.firm }
    
    /// Get read-only access to the message's optional fields.
    pub fn options(&self) -> &Vec<TagValue> {
        &self.optional_appendage.tag_values()
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
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'E');
        bytes.extend(self.user_ref_num.encode());
        bytes.extend(self.firm.encode());
        bytes.extend(self.optional_appendage.encode());

        bytes
    }

    /// Encode the request to a protocol-compliant byte array.
    pub fn to_bytes(&self) -> Vec<u8> { self. encode() }
} 

