
use nsdq_util::define_enum;

// TODO define enum variant pattern in macro
define_enum!{

    RejectReason [2usize] "";

    /*
    [0x0001_u16] QuoteUnavailable "0x0001",
    [0x0002_u16.to_be_bytes()] DestinationClosed,
    [0x0003_u16.to_be_bytes()] InvalidDisplay,
    [0x0004_u16.to_be_bytes()] InvalidMaxFloor,
    [0x0005_u16.to_be_bytes()] InvalidPegType,
    [0x0006_u16.to_be_bytes()] FatFinger,
    [0x0007_u16.to_be_bytes()] Halted,
    [0x0008_u16.to_be_bytes()] IsoNotAllowed,
    [0x0009_u16.to_be_bytes()] InvalidSide,
    [0x000A_u16.to_be_bytes()] ProcessingError,
    [0x000B_u16.to_be_bytes()] CancelPending,
    [0x000C_u16.to_be_bytes()] FirmNotAuthorized,
    [0x000D_u16.to_be_bytes()] InvalidMinQuantity,
    [0x000E_u16.to_be_bytes()] NoClosingReferencePrice,
    [0x000F_u16.to_be_bytes()] Other,

    [&0x0010_u16.to_be_bytes()] CancelNotAllowed,
    [&0x0011_u16.to_be_bytes()] PeggingNotAllowed,
    [&0x0012_u16.to_be_bytes()] CrossedMarket,
    [&0x0013_u16.to_be_bytes()] InvalidQuantity,
    [&0x0014_u16.to_be_bytes()] InvalidCrossOrder,
    [&0x0015_u16.to_be_bytes()] ReplaceNotAllowed,
    [&0x0016_u16.to_be_bytes()] RoutingNotAllowed,
    [&0x0017_u16.to_be_bytes()] InvalidSymbol,
    [&0x0018_u16.to_be_bytes()] Test,
    [&0x0019_u16.to_be_bytes()] LateLocTooAggressive,
    [&0x001A_u16.to_be_bytes()] RetailNotAllowed,
    [&0x001B_u16.to_be_bytes()] InvalidMidpoint,
    [&0x001C_u16.to_be_bytes()] InvalidDestination,
    [&0x001D_u16.to_be_bytes()] InvalidPrice,
    [&0x001E_u16.to_be_bytes()] SharesExceedThreshold,
    [&0x001F_u16.to_be_bytes()] MaximumNotionalValue,

    [&0x0020_u16.to_be_bytes()] RiskAggregateExposureExceeded,
    [&0x0021_u16.to_be_bytes()] RiskMarketImpact,
    [&0x0022_u16.to_be_bytes()] RiskRestrictedStock,
    [&0x0023_u16.to_be_bytes()] RiskShortSellRestricted,
    [&0x0024_u16.to_be_bytes()] RiskIsoNotAllowed,
    [&0x0025_u16.to_be_bytes()] RiskExceedsAdvLimit,
    [&0x0026_u16.to_be_bytes()] RiskFatFinger,
    [&0x0027_u16.to_be_bytes()] RiskLocateRequired,
    [&0x0028_u16.to_be_bytes()] RiskSymbolMessageRate,
    [&0x0029_u16.to_be_bytes()] RiskPortMessageRate,
    [&0x002A_u16.as_be_bytes()] RiskDuplicateMessageRate,
    [&0x002B_u16.to_be_bytes()] RiskShortSellNotAllowed,
    [&0x002C_u16.to_be_bytes()] RiskMarketOrderNotAllowed,
    [&0x002D_u16.to_be_bytes()] RiskPreMarketNotAllowed,
    [&0x002E_u16.to_be_bytes()] RiskPostMarketNotAllowed,
    [&0x002F_u16.to_be_bytes()] RiskShortSellExemptNotAllowed,

    [&0x0030_u16.to_be_bytes()] RiskSingleOrderNotionalExceeded,
    [&0x0031_u16.to_be_bytes()] RiskMaxQuantityExceeded,
    [&0x0032_u16.to_be_bytes()] ShoStateNotAvailable,
    [&0x0033_u16.to_be_bytes()] RiskIpoMarketBuyNotAllowed,
    */
}

/*
use crate::{
    error::BadElementError,
    helper::u16_from_be_bytes
};


/// Self-descriptive reason why an order was rejected.
/// See the OUCH 5.0 specifications (Appendix C).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {

    QuoteUnavailable,
    DestinationClosed,
    InvalidDisplay,
    InvalidMaxFloor,
    InvalidPegType,
    FatFinger,
    Halted,
    IsoNotAllowed,
    InvalidSide,
    ProcessingError,
    CancelPending,
    FirmNotAuthorized,
    InvalidMinQuantity,
    NoClosingReferencePrice,
    Other,

    CancelNotAllowed,
    PeggingNotAllowed,
    CrossedMarket,
    InvalidQuantity,
    InvalidCrossOrder,
    ReplaceNotAllowed,
    RoutingNotAllowed,
    InvalidSymbol,
    Test,
    LateLocTooAggressive,
    RetailNotAllowed,
    InvalidMidpoint,
    InvalidDestination,
    InvalidPrice,
    SharesExceedThreshold,
    MaximumNotionalValue,

    RiskAggregateExposureExceeded,
    RiskMarketImpact,
    RiskRestrictedStock,
    RiskShortSellRestricted,
    RiskIsoNotAllowed,
    RiskExceedsAdvLimit,
    RiskFatFinger,
    RiskLocateRequired,
    RiskSymbolMessageRate,
    RiskPortMessageRate,
    RiskDuplicateMessageRate,
    RiskShortSellNotAllowed,
    RiskMarketOrderNotAllowed,
    RiskPreMarketNotAllowed,
    RiskPostMarketNotAllowed,
    RiskShortSellExemptNotAllowed,

    RiskSingleOrderNotionalExceeded,
    RiskMaxQuantityExceeded,
    ShoStateNotAvailable,
    RiskIpoMarketBuyNotAllowed,
}

impl RejectReason {

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {

        let data = u16_from_be_bytes(data)?;

        use RejectReason::*;
        match data {

            _ => Err(BadElementError::InvalidEnum(
                format!("{:#x}", data),
                "RejectReason".to_string()
            ))
        }
    }

    #[allow(dead_code)] // Future use
    pub(crate) fn encode(&self) -> [u8; 2] {
        
        use RejectReason::*;
        let data: u16 = match self {

             QuoteUnavailable                   => 0x0001,
             DestinationClosed                  => 0x0002,
             InvalidDisplay                     => 0x0003,
             InvalidMaxFloor                    => 0x0004,
             InvalidPegType                     => 0x0005,
             FatFinger                          => 0x0006,
             Halted                             => 0x0007,
             IsoNotAllowed                      => 0x0008,
             InvalidSide                        => 0x0009,
             ProcessingError                    => 0x000A,
             CancelPending                      => 0x000B,
             FirmNotAuthorized                  => 0x000C,
             InvalidMinQuantity                 => 0x000D,
             NoClosingReferencePrice            => 0x000E,
             Other                              => 0x000F,

             CancelNotAllowed                   => 0x0010,
             PeggingNotAllowed                  => 0x0011,
             CrossedMarket                      => 0x0012,
             InvalidQuantity                    => 0x0013,
             InvalidCrossOrder                  => 0x0014,
             ReplaceNotAllowed                  => 0x0015,
             RoutingNotAllowed                  => 0x0016,
             InvalidSymbol                      => 0x0017,  
             Test                               => 0x0018,
             LateLocTooAggressive               => 0x0019,
             RetailNotAllowed                   => 0x001A,
             InvalidMidpoint                    => 0x001B,
             InvalidDestination                 => 0x001C,
             InvalidPrice                       => 0x001D,
             SharesExceedThreshold              => 0x001E,

             MaximumNotionalValue               => 0x002F,
             RiskAggregateExposureExceeded      => 0x0020,
             RiskMarketImpact                   => 0x0021,
             RiskRestrictedStock                => 0x0022,
             RiskShortSellRestricted            => 0x0023,
             RiskIsoNotAllowed                  => 0x0024,
             RiskExceedsAdvLimit                => 0x0025,
             RiskFatFinger                      => 0x0026,
             RiskLocateRequired                 => 0x0027,
             RiskSymbolMessageRate              => 0x0028,
             RiskPortMessageRate                => 0x0029,
             RiskDuplicateMessageRate           => 0x002A,
             RiskShortSellNotAllowed            => 0x002B,
             RiskMarketOrderNotAllowed          => 0x002C,
             RiskPreMarketNotAllowed            => 0x002D,
             RiskPostMarketNotAllowed           => 0x002E,
             RiskShortSellExemptNotAllowed      => 0x002F,

             RiskSingleOrderNotionalExceeded    => 0x0030,
             RiskMaxQuantityExceeded            => 0x0031,
             ShoStateNotAvailable               => 0x0032,
             RiskIpoMarketBuyNotAllowed         => 0x0033,
        };

        data.to_be_bytes()
    }

}
*/
