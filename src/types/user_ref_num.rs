
use crate::{
    error::BadElementError,
    helper::u32_from_be_bytes
};


/// Unsigned number used to differentiate requests.
/// These are day-unique and must be strictly increasing for each new identifier.
///
/// There is no clean way to initialize a UserRefNum with an arbitrary value,
/// which is intentional, to prevent accidental reuse of identifiers.
///
/// The `OuchClient` from this crate will automatically increment 
/// the `UserRefNum` for each request that requires a unique number.
///
/// If you will need to refer to an order at a later time, 
/// be sure to copy and save its UserRefNum somewhere in your code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserRefNum(u32);

impl UserRefNum {

    /// Fresh number to start the day.
    /// UserRefNum begins at 1.
    pub fn new() -> Self {
        UserRefNum(1u32)
    }

    /// Used by client.
    pub fn increment(&mut self) {
        // TODO: Check for overflow?
        self.0 += 1
    }

    /// Gets the value as a u32.
    pub fn val(&self) -> u32 { self.0 }

    pub(crate) fn encode(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    /// Parse a UserRefNum from an array of 4 big-endian bytes.
    ///
    /// There is no clean way to initialize a UserRefNum with an arbitrary value,
    /// which is intentional, to prevent accidental reuse of identifiers.
    /// If you absolutely need to create a UserRefNum with an arbitrary value,
    /// make sure you do so safely.
    /// This method is made public to cover unforeseen situations 
    /// but should not be relied upon by design.
    pub fn parse(data: &[u8]) -> Result<Self, BadElementError> {

        let val = u32_from_be_bytes(data)?;
        Ok(UserRefNum(val))
    }
}


