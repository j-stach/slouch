
use nsdq_util::error::TypeError;

/// Used to set `TagValue::ExpireTime`.
/// Seconds to live. Must be less than 86400 (number of seconds in a day).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElapsedTime(u32);

impl ElapsedTime {
    
    /// Create a new time limit for use with `TagValue::ExpireTime`.
    /// `secs` must be less than 86400 (number of seconds in a day).
    pub fn new(secs: u32) -> Result<Self, TypeError> {
        
        if secs >= 86_400 {
            return Err(TypeError::InvalidTime((secs as u64) * 10u64.pow(9)))
        }

        Ok(Self(secs))
    }

    /// Number of seconds to live.
    pub fn secs(&self) -> u32 { self.0 }

    /// Number of nanoseconds to live.
    pub fn nanosecs(&self) -> u64 { 
        (self.0 as u64) * 10u64.pow(9)
    }

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


