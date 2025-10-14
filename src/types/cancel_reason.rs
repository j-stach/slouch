
use crate::error::BadElementError;

/// Reason an order was cancelled (`CancelOrder` message).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {

    /// The order cannot be executed because of a regulatory restriction
    /// (e.g., trade through restrictions).
    RegulatoryRestriction,

    /// Any `TimeInForce::Day` order that was received after the closing
    /// cross is complete in a given symbol will receive this reason.
    Closed,

    /// Post Only order would have been price slid for NMS.
    PostOnlyNms,

    /// Post Only order would have been price slid due to a 
    /// contra side displayed order on the book.
    PostOnlySide,

    /// On-open order was canceled because the symbol remained halted 
    /// after the opening cross completed.
    Halted,

    /// Immediate or Cancel Order.
    Immediate,

    /// This order cannot be executed because of Market Collars.
    MarketCollars,

    /// The order was cancelled because it would have executed with an 
    /// existing order entered by the same MPID.
    SelfMatchPrevention,

    /// The order was manually canceled by a NASDAQ supervisory terminal.
    Supervisory,

    /// The Time In Force for the order has expired.
    Timeout,

    /// Sent in response to a CancelOrder request.
    UserRequested,

    /// The order was cancelled by the Opening Price Protection Threshold.
    OpenProtection,

    /// This order was cancelled by the system.
    System,

    /// Company Direct Listing Capital Raise order exceeds allowable shares.
    ExceedsAllowableShares,

}

impl CancelReason {

    pub(crate) fn parse(data: u8) -> Result<Self, BadElementError> {

        use CancelReason::*;
        match data {

            b'D' => Ok(RegulatoryRestriction),
            b'E' => Ok(Closed),
            b'F' => Ok(PostOnlyNms),
            b'G' => Ok(PostOnlySide),
            b'H' => Ok(Halted),
            b'I' => Ok(Immediate),
            b'K' => Ok(MarketCollars),
            b'Q' => Ok(SelfMatchPrevention),
            b'S' => Ok(Supervisory),
            b'T' => Ok(Timeout),
            b'U' => Ok(UserRequested),
            b'X' => Ok(OpenProtection),
            b'Z' => Ok(System),
            b'e' => Ok(ExceedsAllowableShares),

            _ => Err(BadElementError::InvalidEnum(
                (data as char).to_string(), 
                "CancelReason".to_string()
            ))
        }
    }

    #[allow(dead_code)] // Future use
    pub(crate) fn encode(&self) -> u8 {
        
        use CancelReason::*;
        match self {

             RegulatoryRestriction   => b'D',
             Closed                  => b'E',
             PostOnlyNms             => b'F',
             PostOnlySide            => b'G',
             Halted                  => b'H',
             Immediate               => b'I',
             MarketCollars           => b'K',
             SelfMatchPrevention     => b'Q',
             Supervisory             => b'S',
             Timeout                 => b'T',
             UserRequested           => b'U',
             OpenProtection          => b'X',
             System                  => b'Z',
             ExceedsAllowableShares  => b'e',

        }
    }

}

