
use crate::{
    error::BadElementError,
    helper::u16_from_be_bytes
};


///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {

    /// Test only
    Test,

    /// Late LOC Too Aggressive
    LateLocTooAggressive,

    /// Retail Not Allowed
    RetailNotAllowed,

    /// Invalid Midpoint Post Only Price
    InvalidMidpoint,

    /// Invalid Destination
    InvalidDestination,

    /// Invalid Price
    InvalidPrice,

    /// Shares Exceed Threshold
    SharesExceedThreshold,

    /// Exceeds Maximum Allowed Notional Value
    MaximumNotionalValue,

    /// Risk: Aggregate Exposure Exceeded
    AggregateExposureExceeded,

    /// Risk: Market Impact
    MarketImpact,

    /// Risk: Restricted Stock
    RestrictedStock,

    /// Risk: Short Sell Restricted
    ShortSellRestricted,

    /// Risk: ISO Not Allowed
    IsoNotAllowed,

    /// Risk: Exceeds ADV Limit
    ExceedsAdvLimit,

    /// Risk: Fat Finger
    FatFinger,

    /// Risk: Locate Required
    LocateRequired,

    /// Risk: Symbol Message Rate Restriction
    SymbolMessageRate,

    /// Risk: Port Message Rate Restriction
    PortMessageRate,

    /// Risk: Duplicate Message Rate Restriction
    DuplicateMessageRate,

    /// Risk: Short Sell Not Allowed
    ShortSellNotAllowed,

    /// Risk: Market Order Not Allowed
    MarketOrderNotAllowed,

    /// Risk: Pre-Market Not Allowed
    PreMarketNotAllowed,

    /// Risk: Post-Market Not Allowed
    PostMarketNotAllowed,

    /// Risk: Short Sell Exempt Not Allowed
    ShortSellExemptNotAllowed,

    /// Risk: Single Order Notional Exceeded
    SingleOrderNotionalExceeded,

    /// Risk: Max Quantity Exceeded
    MaxQuantityExceeded,

    /// Risk: SHO State Not Available
    ShoStateNotAvailable,

    /// IPO Market Buy Not Allowed
    IpoMarketBuyNotAllowed,

}

impl RejectReason {

    pub(crate) fn parse(data: &[u8]) -> Result<Self, BadElementError> {

        let data = u16_from_be_bytes(data)?;

        use RejectReason::*;
        match data {

            0x0018 => Ok(Test),
            0x0019 => Ok(LateLocTooAggressive),
            0x001A => Ok(RetailNotAllowed),
            0x001B => Ok(InvalidMidpoint),
            0x001C => Ok(InvalidDestination),
            0x001D => Ok(InvalidPrice),
            0x001E => Ok(SharesExceedThreshold),
            0x002F => Ok(MaximumNotionalValue),
            0x0020 => Ok(AggregateExposureExceeded),
            0x0021 => Ok(MarketImpact),
            0x0022 => Ok(RestrictedStock),
            0x0023 => Ok(ShortSellRestricted),
            0x0024 => Ok(IsoNotAllowed),
            0x0025 => Ok(ExceedsAdvLimit),
            0x0026 => Ok(FatFinger),
            0x0027 => Ok(LocateRequired),
            0x0028 => Ok(SymbolMessageRate),
            0x0029 => Ok(PortMessageRate),
            0x002A => Ok(DuplicateMessageRate),
            0x002B => Ok(ShortSellNotAllowed),
            0x002C => Ok(MarketOrderNotAllowed),
            0x002D => Ok(PreMarketNotAllowed),
            0x002E => Ok(PostMarketNotAllowed),
            0x003F => Ok(ShortSellExemptNotAllowed),
            0x0030 => Ok(SingleOrderNotionalExceeded),
            0x0031 => Ok(MaxQuantityExceeded),
            0x0032 => Ok(ShoStateNotAvailable),
            0x0033 => Ok(IpoMarketBuyNotAllowed),

            _ => Err(BadElementError::InvalidEnum(
                format!("{:#x}", data),
                "RejectReason".to_string()
            ))
        }
    }

    pub(crate) fn encode(&self) -> [u8; 2] {
        
        use RejectReason::*;
        let data: u16 = match self {

             Test                          => 0x0018,
             LateLocTooAggressive          => 0x0019,
             RetailNotAllowed              => 0x001A,
             InvalidMidpoint               => 0x001B,
             InvalidDestination            => 0x001C,
             InvalidPrice                  => 0x001D,
             SharesExceedThreshold         => 0x001E,
             MaximumNotionalValue          => 0x002F,
             AggregateExposureExceeded     => 0x0020,
             MarketImpact                  => 0x0021,
             RestrictedStock               => 0x0022,
             ShortSellRestricted           => 0x0023,
             IsoNotAllowed                 => 0x0024,
             ExceedsAdvLimit               => 0x0025,
             FatFinger                     => 0x0026,
             LocateRequired                => 0x0027,
             SymbolMessageRate             => 0x0028,
             PortMessageRate               => 0x0029,
             DuplicateMessageRate          => 0x002A,
             ShortSellNotAllowed           => 0x002B,
             MarketOrderNotAllowed         => 0x002C,
             PreMarketNotAllowed           => 0x002D,
             PostMarketNotAllowed          => 0x002E,
             ShortSellExemptNotAllowed     => 0x003F,
             SingleOrderNotionalExceeded   => 0x0030,
             MaxQuantityExceeded           => 0x0031,
             ShoStateNotAvailable          => 0x0032,
             IpoMarketBuyNotAllowed        => 0x0033,
        };

        data.to_be_bytes()
    }

}
