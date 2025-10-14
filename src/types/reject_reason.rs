
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

            0x0001 => Ok(QuoteUnavailable),
            0x0002 => Ok(DestinationClosed),
            0x0003 => Ok(InvalidDisplay),
            0x0004 => Ok(InvalidMaxFloor),
            0x0005 => Ok(InvalidPegType),
            0x0006 => Ok(FatFinger),
            0x0007 => Ok(Halted),
            0x0008 => Ok(IsoNotAllowed),
            0x0009 => Ok(InvalidSide),
            0x000A => Ok(ProcessingError),
            0x000B => Ok(CancelPending),
            0x000C => Ok(FirmNotAuthorized),
            0x000D => Ok(InvalidMinQuantity),
            0x000E => Ok(NoClosingReferencePrice),
            0x000F => Ok(Other),

            0x0010 => Ok(CancelNotAllowed),
            0x0011 => Ok(PeggingNotAllowed),
            0x0012 => Ok(CrossedMarket),
            0x0013 => Ok(InvalidQuantity),
            0x0014 => Ok(InvalidCrossOrder),
            0x0015 => Ok(ReplaceNotAllowed),
            0x0016 => Ok(RoutingNotAllowed),
            0x0017 => Ok(InvalidSymbol),
            0x0018 => Ok(Test),
            0x0019 => Ok(LateLocTooAggressive),
            0x001A => Ok(RetailNotAllowed),
            0x001B => Ok(InvalidMidpoint),
            0x001C => Ok(InvalidDestination),
            0x001D => Ok(InvalidPrice),
            0x001E => Ok(SharesExceedThreshold),
            0x001F => Ok(MaximumNotionalValue),

            0x0020 => Ok(RiskAggregateExposureExceeded),
            0x0021 => Ok(RiskMarketImpact),
            0x0022 => Ok(RiskRestrictedStock),
            0x0023 => Ok(RiskShortSellRestricted),
            0x0024 => Ok(RiskIsoNotAllowed),
            0x0025 => Ok(RiskExceedsAdvLimit),
            0x0026 => Ok(RiskFatFinger),
            0x0027 => Ok(RiskLocateRequired),
            0x0028 => Ok(RiskSymbolMessageRate),
            0x0029 => Ok(RiskPortMessageRate),
            0x002A => Ok(RiskDuplicateMessageRate),
            0x002B => Ok(RiskShortSellNotAllowed),
            0x002C => Ok(RiskMarketOrderNotAllowed),
            0x002D => Ok(RiskPreMarketNotAllowed),
            0x002E => Ok(RiskPostMarketNotAllowed),
            0x002F => Ok(RiskShortSellExemptNotAllowed),

            0x0030 => Ok(RiskSingleOrderNotionalExceeded),
            0x0031 => Ok(RiskMaxQuantityExceeded),
            0x0032 => Ok(ShoStateNotAvailable),
            0x0033 => Ok(RiskIpoMarketBuyNotAllowed),

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
