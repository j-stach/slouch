
use crate::{
    error::BadElementError,
    helper::u32_from_be_bytes
};


/// Unsigned number used to differentiate requests.
/// These are day-unique and must be strictly increasing.
/// The `OuchClient` from this crate will automatically increment 
/// the `UserRefNum` for each new request.
pub struct UserRefNum(u32);

impl UserRefNum {

    /// Fresh number to start the day.
    pub fn new() -> Self {
        UserRefNum(0u32)
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        self.0.to_be_bytes()
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, BadElementError> {

        let val = u32_from_be_bytes(data)?;
        Ok(UserRefNum(val))
    }
}


