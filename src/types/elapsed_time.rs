
use crate::error::BadElementError;

/// Used to set `TagValue::ExpireTime`.
/// Seconds to live. Must be less than 86400 (number of seconds in a day).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElapsedTime(u32);

impl ElapsedTime {
    
    /// Create a new time limit for use with `TagValue::ExpireTime`.
    /// `secs` must be less than 86400 (number of seconds in a day).
    pub fn new(secs: u32) -> Result<Self, BadElementError> {
        
        if secs >= 86_400 {
            return Err(
                BadElementError::InvalidValue("ElapsedTime".to_string())
            )
        }

        Ok(Self(secs))
    }

    pub fn to_secs(&self) -> u32 { self.0 }

    pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

        let (input, secs) = nom::number::streaming::be_u32(input)?;
        let time = Self::new(secs)
            .expect("Should only need to check user-created values.");

        Ok((input, time))
    }

    // Limits should have been checked when ElapsedTime was created.
    pub(crate) fn encode(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}


