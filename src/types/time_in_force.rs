
use crate::error::BadElementError;

/// During which time span will the order be active on the book?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {

    /// Active during regular market hours on the day it is submitted.
    Day,

    /// "IOC" 
    /// Must execute immediately, either fully or partially, 
    /// with any unfilled portion canceled. 
    /// It does not rest in the order book, prioritizing speed over persistence.
    ImmediateOrCancel,

    /// "GTX"
    /// Active during extended hours, including pre-market, regular market hours,
    /// and after-hours. 
    /// If unexecuted by the end of extended hours, it is canceled.
    GoodTilExtended,

    /// "GTT"
    /// Order that remains active until a user-defined expiration time, 
    /// to be provided via the `TagValue::ExpireTime` option.
    /// NOTE: ExpireTime is required on orders with this TiF.
    GoodTilTime,

    /// Only active after hours.
    AfterHours
}

impl TimeInForce {

    pub(crate) fn encode(&self) -> u8 {

        use TimeInForce::*;
        match self {
            Day => b'0', 
            ImmediateOrCancel => b'3', 
            GoodTilExtended => b'5', 
            GoodTilTime => b'6', 
            AfterHours   => b'E', 
        }
    }

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use TimeInForce::*;
        match data {
            b'0' => Ok(Day),
            b'3' => Ok(ImmediateOrCancel),
            b'5' => Ok(GoodTilExtended),
            b'6' => Ok(GoodTilTime),
            b'E' => Ok(AfterHours),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "TimeInForce".to_string()
            ))
        }
    }
}
