
use nsdq_util::{
    define_str,
    types::string::helper::*,
};

use nsdq_util::error::TypeError;

define_str!{
    RouteId [4usize] 
        "Strong type for route IDs that ensures protocol compliance."
}

impl RouteId {

    /// Generate a new RouteId from a protocol-compliant string.
    /// May only contain ASCII alphabetic characters.
    /// Only up to the first 14 characters will be included, others ignored.
    pub fn from(s: impl AsRef<str>) -> Result<Self, TypeError> {

        let s = s.as_ref();

        if !is_alpha(s) {
            return Err(
                TypeError::InvalidString(
                    "RouteId".to_string(), 
                    s.to_string()
                )
            );
        }

        Ok(RouteId(fixed_str::<4usize>(s)))
    }
}

