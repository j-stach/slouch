
use crate::error::BadElementError;
use crate::helper::u32_from_be_bytes;

/// Used to set `TagValue::ExpireTime`.
/// Seconds to live. Must be less than 86400 (number of seconds in a day).
pub struct ElapsedTime(u32);

impl ElapsedTime {
    
    /// Create a new time limit for `TagValue::ExpireTime`.
    /// `secs` must be less than 86400 (number of seconds in a day).
    pub fn new(secs: u32) -> Result<Self, BadElementError> {
        
        if secs >= 86_400 {
            return Err(
                BadElementError::InvalidValue("ElapsedTime".to_string())
            )
        }

        Self(secs)
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {
        let secs = u32_from_be_bytes(data)?;
        Self::new(secs)
    }

    // Limits should have been checked when ElapsedTime was created.
    pub(crate) fn encode(&self) -> Vec<u8> {
        self.0.to_be_bytes()
    }
}


