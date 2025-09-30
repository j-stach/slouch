
use crate::msg::options::{
    OptionalAppendage,
    TagValue
};

///
#[derive(Debug, Clone)]
pub struct AccountQuery {
    /// This holds all optional fields included in the order.
    optional_appendage: OptionalAppendage
}

impl AccountQuery {
    
    pub(super) fn encode(&self) -> Vec<u8> {

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(b'Q');
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
                    "AccountQuery".to_string()
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

