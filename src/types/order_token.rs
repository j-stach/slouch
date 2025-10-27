
use nsdq_util::{
    define_str,
    types::string::helper::*,
};

use crate::error::BadElementError;

define_str!{
    OrderToken [14usize] 
        "Strong type for `ClOrdId` that ensures protocol compliance.
        Can be used to differentiate strategies, etc.
        Will not be checked for day-uniqueness for each OUCH account."
}


impl OrderToken {

    /// Generate a new OrderToken from a protocol-compliant string.
    /// May only contain ASCII alphanumeric characters and spaces.
    /// Only up to the first 14 characters will be included, others ignored.
    pub fn from(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();

        if !is_alphanumeric(s) {
            return Err(
                BadElementError::NotAlpha("OrderToken".to_string())
            );
        }

        Ok(OrderToken(fixed_str::<14usize>(s)))
    }
}

