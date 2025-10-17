
use crate::{
    error::{ OuchError, BadElementError },
    helper::{
        u16_from_be_bytes,
        u32_from_be_bytes,
        u64_from_be_bytes,
    }
};

use crate::types::{
    FirmId,
    CustomerType,
    PriceType,
    Price,
    SignedPrice,
    HandleInst,
    BboWeightIndicator,
    RouteId,
    BrokerId,
    ElapsedTime,
    Side,
    TradeNow
};


/// An optional attribute on an order is communicated via a TagValue element.
///
/// These names are kept as similar as practical to the corresponding options
/// as documented in the OUCH 5.0 specifications.
///
/// If an optional field is not provided on a message, 
/// OUCH will use the default value (see Appendix A in the spec).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagValue {

    /// An alternative order reference number used when publishing the order 
    /// on the NASDAQ market data feeds 
    /// (identifying, for example, the displayed portion of a reserve order).
    SecondaryOrdRefNum(u64),

    /// Identifier for the firm placing the order.
    Firm(FirmId),

    /// Must be a round lot.
    MinQty(u32),

    /// Specifies the type of customer for the order 
    /// (e.g., retail, institutional).
    CustomerType(CustomerType),

    /// Represents the portion of your order that you wish to have displayed.
    MaxFloor(u32),

    /// Specifies the type of pricing for the order (e.g., limit, market).
    PriceType(PriceType),

    /// Offset amount for the pegged value.
    PegOffset(SignedPrice),

    /// Price for discretionary order execution.
    DiscretionPrice(Price),

    /// Limited use of `PriceType`: cant use `MarketMakerPeg` or `Midpoint`.
    DiscretionPriceType(PriceType),

    /// Offset amount for the pegged value of the Discretionary Price.
    DiscretionPegOffset(SignedPrice),

    /// Indicates if the order is post-only (will not execute immediately).
    PostOnly(bool),

    /// Shares to do random reserve with.
    RandomReserves(u32),

    /// Specifies the routing destination for the order.
    Route(RouteId),

    /// Seconds to live. 
    /// Must be less than 86400 (number of seconds in a day).
    ExpireTime(ElapsedTime),

    /// Indicates if the order should be executed immediately.
    TradeNow(TradeNow),

    /// Specifies handling instructions for the order 
    /// (e.g., automated execution). 
    HandleInst(HandleInst),

    /// Indicates the weighting of the order in the Best Bid and Offer (BBO). 
    BboWeightIndicator(BboWeightIndicator),

    /// Used in the Order Restated Message only.
    /// Represents an update of an order’s displayed quantity 
    /// (i.e. an order with reserves).
    DisplayQuantity(u32),

    /// Used in the Order Restated Message only.
    /// Represents an update of an order’s displayed price.
    DisplayPrice(Price),

    /// Customer Group ID – identifies specific entity.
    GroupId(u16),

    /// Shares located for short sale order.
    SharesLocated(bool),

    /// Broker code from which the locate has been acquired 
    /// for short sale orders.
    LocateBroker(BrokerId),

    /// Specifies the side of the order (e.g., buy or sell). 
    Side(Side),

    /// User Reference Index - identifies the channel within the given port.
    UserRefIndex(u8),

}

impl TagValue {

    pub(crate) fn encode(&self) -> Vec<u8> {

        use TagValue::*;
        // Take note of missing tags => 8, 19-21
        let (option_tag, encoded_value) = match self {
            
            SecondaryOrdRefNum(val)     => (1,  val.to_be_bytes().to_vec()),
            Firm(val)                   => (2,  val.encode().to_vec()),
            MinQty(val)                 => (3,  val.to_be_bytes().to_vec()),
            CustomerType(val)           => (4,  vec![val.encode()]),
            MaxFloor(val)               => (5,  val.to_be_bytes().to_vec()),
            PriceType(val)              => (6,  vec![val.encode()]),
            PegOffset(val)              => (7,  val.encode().to_vec()),
            DiscretionPrice(val)        => (9,  val.encode().to_vec()),
            DiscretionPriceType(val)    => (10, vec![val.encode()]),
            DiscretionPegOffset(val)    => (11, val.encode().to_vec()),
            PostOnly(val)               => (12, {
                vec![ match val {
                    true => b'P',
                    false => b'N',
                }]
            }), 
            RandomReserves(val)         => (13, val.to_be_bytes().to_vec()),
            Route(val)                  => (14, val.encode().to_vec()),
            ExpireTime(val)             => (15, val.encode().to_vec()), 
            TradeNow(val)               => (16, vec![val.encode()]),
            HandleInst(val)             => (17, vec![val.encode()]),
            BboWeightIndicator(val)     => (18, vec![val.encode()]),
            DisplayQuantity(val)        => (22, val.to_be_bytes().to_vec()),
            DisplayPrice(val)           => (23, val.encode().to_vec()),
            GroupId(val)                => (24, val.to_be_bytes().to_vec()),
            SharesLocated(val)          => (25, {
                vec![ match val {
                    true => b'Y',
                    false => b'N',
                }]
            }), 
            LocateBroker(val)           => (26, val.encode().to_vec()),
            Side(val)                   => (27, vec![val.encode()]),
            UserRefIndex(val)           => (28, val.to_be_bytes().to_vec()),
        };

        // Start encoded array with length (calculated, not tracked);
        // this is the start of the protocol's `TagValue` type.
        // Safely assume that length will be less than u8::MAX,
        // because each enum variant's inner type encodes to less.
        let length: u8 = encoded_value.len() as u8 + 1;

        let mut data = vec![length, option_tag as u8];
        data.extend(encoded_value);

        // This is formatted as the protocol's TagValue, 
        // including length marker:
        data
    }

    pub(crate) fn parse(data: &[u8]) -> Result<Self, OuchError> {

        // Length-marking byte of tag is not added to `data` when 
        // `OptionalAppendage::parse` calls this method.
        let option_tag = data[0];
        let payload = &data[1..];

        match option_tag {

            1 => Ok(Self::SecondaryOrdRefNum(u64_from_be_bytes(payload)?)), 
            2 => Ok(Self::Firm(FirmId::parse(payload)?)),
            3 => Ok(Self::MinQty(u32_from_be_bytes(payload)?)),
            4 => Ok(Self::CustomerType(CustomerType::parse(payload[0])?)),
            5 => Ok(Self::MaxFloor(u32_from_be_bytes(payload)?)),
            6 => Ok(Self::PriceType(PriceType::parse(payload[0])?)),
            7 => Ok(Self::PegOffset(SignedPrice::parse(payload)?)),
            9 => Ok(Self::DiscretionPrice(Price::parse(payload)?)),
            10 => {
                let price_type = PriceType::parse(payload[0])?;
                use PriceType::*;
                match price_type {
                    // Valid PriceType for DiscretionPriceType 
                    // excludes "Q" and "m".
                    MarketMakerPeg | Midpoint => Err(
                        BadElementError::InvalidEnum(
                            (price_type.encode() as char).to_string(), 
                            "DiscretionPriceType".to_string()
                        ).into()
                    ),
                    _ => Ok(Self::DiscretionPriceType(price_type))
                }
            },

            11 => Ok(Self::DiscretionPegOffset(SignedPrice::parse(payload)?)),
            12 => {
                let val = payload[0];
                match val {
                    b'P' => Ok(Self::PostOnly(true)),
                    b'N' => Ok(Self::PostOnly(false)),

                    _ => Err(BadElementError::InvalidEnum(
                            (val as char).to_string(), 
                            "PostOnly".to_string()
                        ).into())
                }
            },

            13 => Ok(Self::RandomReserves(u32_from_be_bytes(payload)?)),
            14 => Ok(Self::Route(RouteId::parse(payload)?)),
            15 => Ok(Self::ExpireTime(ElapsedTime::parse(payload)?)),
            16 => Ok(Self::TradeNow(TradeNow::parse(payload[0])?)),
            17 => Ok(Self::HandleInst(HandleInst::parse(payload[0])?)),
            18 => Ok(Self::BboWeightIndicator(
                    BboWeightIndicator::parse(payload[0])?
            )),

            22 => Ok(Self::DisplayQuantity(u32_from_be_bytes(payload)?)),
            23 => Ok(Self::DisplayPrice(Price::parse(payload)?)),
            24 => Ok(Self::GroupId(u16_from_be_bytes(payload)?)),
            25 => {
                let val = payload[0];
                match val {
                    b'Y' => Ok(Self::SharesLocated(true)),
                    b'N' => Ok(Self::SharesLocated(false)),

                    _ => Err(BadElementError::InvalidEnum(
                            (val as char).to_string(), 
                            "SharesLocated".to_string()
                        ).into())
                }
            },

            26 => Ok(Self::LocateBroker(BrokerId::parse(payload)?)),
            27 => Ok(Self::Side(Side::parse(payload[0])?)),
            28 => Ok(Self::UserRefIndex(payload[0])),

            _ => Err(BadElementError::InvalidEnum(
                (option_tag as char).to_string(), 
                "TagValue".to_string()
            ).into()),
        }
    }

}
