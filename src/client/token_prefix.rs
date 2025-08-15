
use std::fmt;
use derive_more::{ Deref, DerefMut };
use serde::{ Deserialize, Serialize };

use crate::helper::check_string_compliance;
use crate::error::BadElementError;


/// Strong type for order token prefixes that ensures protocol compliance.
#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct TokenPrefix(String);

impl TokenPrefix {

    /// Generate a new TokenPrefix from a protocol-compliant string.
    /// The entire `OrderToken` must be fewer than 14 bytes, 
    /// so if you want to take full advantage of the token generators,
    /// you should leave at least 6 bytes to encode the order number.
    /// The shorter the prefix used, the more orders that can be placed 
    /// before the sequence tracker rolls over.
    pub fn new(s: impl AsRef<str>) -> Result<Self, BadElementError> {

        let s = s.as_ref();
        check_string_compliance(s, 14, "TokenPrefix")?;

        Ok(TokenPrefix(s.to_string()))
    }
}

impl fmt::Display for TokenPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

