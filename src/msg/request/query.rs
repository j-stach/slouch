
use nom::number::streaming::{ be_u32, be_u64 };

use crate::error::BadElementError;
use crate::{ types::*, msg::define_msg };


/// Create an AccountQuery message.
/// ```
/// use slouch::account_query;
/// let request1 = account_query!();
///
/// use slouch::msg::{ OuchRequest, AccountQuery };
/// let request2 = OuchRequest::AccountQuery(AccountQuery::new());
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! account_query {
    () => {
        $crate::msg::OuchRequest::AccountQuery($crate::msg::AccountQuery::new())
    }
}

define_msg!{
    AccountQuery: 
    "Use when recovering state to request the next available UserRefNum.";
}

impl AccountQuery {

    /// Create a new Account Query.
    pub fn new() -> Self {
        Self {
            optional_appendage: crate::msg::options::OptionalAppendage::new(),
        }
    }

    /*
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
                    "AccountQuery".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
} 

