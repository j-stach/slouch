
use crate::{
    error::OuchError,
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
    Route,
    Broker,
    Side
};


/// 
/// These names are kept as similar as possible to the corresponding options
/// as documented in the OUCH 5.0 specifications.
#[derive(Debug, Serialize, Deserialize)]
pub enum TagValue {

    /// An alternative order reference number used when publishing the order 
    /// on the NASDAQ market data feeds 
    /// (identifying, for example, the displayed portion of a reserve order).
    // TODO: Send email to ask what type "Q" is, 
    // and if it needs any special treatment.
    SecondaryOrdRefNum(u64),

    /// 
    Firm(FirmId),

    /// Must be a round lot.
    MinQty(u32),

    ///
    CustomerType(CustomerType),

    /// Represents the portion of your order that you wish to have displayed.
    MaxFloor(u32),

    ///
    PriceType(PriceType),

    /// Offset amount for the pegged value.
    PegOffset(SignedPrice),

    ///
    DiscretionPrice(Price),

    /// Limited use of `PriceType`: cant use `MarketMakerPeg` or `Midpoint`.
    DiscretionPriceType(PriceType),

    /// Offset amount for the pegged value of the Discretionary Price.
    DiscretionPegOffset(SignedPrice),

    ///
    PostOnly(bool),

    /// Shares to do random reserve with.
    RandomReserves(u32),

    ///
    // TODO: Send email to ask if there are any restrictions to this.
    Route(Route),

    /// Seconds to live. 
    /// Must be less than 86400 (number of seconds in a day).
    // TODO: Limited duration using secs
    ExpireTime(u32),

    ///
    TradeNow(bool),

    /// 
    HandleInst(HandleInst),

    /// 
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
    LocateBroker(Broker),

    /// 
    Side(Side),

    /// User Reference Index - identifies the channel within the given port.
    UserRefIndex(u8),

}

impl TagValue {

    pub(crate) fn encode(&self) -> Vec<u8> {

        // Take note of missing tags => 8, 19-21
        let (option_tag: u8, encoded_value: Vec<u8>) = match self {
            
            SecondaryOrdRefNum(val)     => (1,  val.to_be_bytes()),
            Firm(val)                   => (2,  val.encode()),
            MinQty(val)                 => (3,  val.to_be_bytes()),
            CustomerType(val)           => (4,  val.encode()),
            MaxFloor(val)               => (5,  val.to_be_bytes()),
            PriceType(val)              => (6,  val.encode()),
            PegOffset(val)              => (7,  val.encode()),
            DiscretionPrice(val)        => (9,  val.encode()),
            DiscretionPriceType(val)    => (10, val.encode()),
            DiscretionPegOffset(val)    => (11, val.encode()),
            PostOnly(val)               => (12, {
                let data = match val {
                    true => b'P'.to_be_bytes(),
                    false => b'N'.to_be_bytes(),
                };
                vec![data]
            }), 
            RandomReserves(val)         => (13, val.to_be_bytes()),
            Route(val)                  => (14, val.encode()),
            ExpireTime(val)             => (15, val.to_be_bytes()), //TODO
            TradeNow(val)               => (16, {
                let data = match val {
                    true => b'Y'.to_be_bytes(),
                    false => b'N'.to_be_bytes(),
                };
                vec![data]
            }), 
            HandleInst(val)             => (17, val.encode()),
            BboWeightIndicator(val)     => (18, val.encode()),
            DisplayQuantity(val)        => (22, val.to_be_bytes()),
            DisplayPrice(val)           => (23, val.encode()),
            GroupId(val)                => (24, val.to_be_bytes()),
            SharesLocated(val)          => (25, {
                let data = match val {
                    true => b'Y'.to_be_bytes(),
                    false => b'N'.to_be_bytes(),
                };
                vec![data]
            }), 
            LocateBroker(val)           => (26, val.encode()),
            Side(val)                   => (27, val.encode()),
            UserRefIndex(val)           => (28, val.to_be_bytes()),
        };

        // Start encoded array with length (calculated, not tracked);
        // this is the start of the protocol's `TagValue` type.
        // Safely assume that length will be less than u8::MAX,
        // because each enum variant's inner type encodes to less.
        let length: u8 = (encoded_value.len() as u8 + 1).to_be_bytes();

        let mut data = vec![length, option_tag];
        data.extend(encoded_value);

        // This is formatted as the protocol's TagValue, 
        // including length marker:
        data
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {

        // Length-marking byte of tag is not added to `data` when 
        // `OptionalAppendage::parse` calls this method.
        let option_tag = u8::from_be_bytes(data[0]);
        let payload = &data[1..];

        match option_tag {

            1 => Ok(u64_from_be_bytes(payload)? 
                .map(Self::SecondaryOrdRefNum)),

            2 => Ok(FirmId::parse(payload)?
                .map(Self::Firm)),

            3 => Ok(u32_from_be_bytes(payload)?
                .map(Self::MinQty)),

            4 => Ok(CustomerType::parse(payload)?
                .map(Self::CustomerType)),

            5 => Ok(u34_from_be_bytes(payload)?
                .map(Self::MaxFloor)),

            6 => Ok(PriceType::parse(payload)?
                .map(Self::PriceType)),

            7 => Ok(SignedPrice::parse(payload)?
                .map(Self::PegOffset)),

            9 => Ok(Price::parse(payload)?
                .map(Self::DiscretionPrice)),

            10 => {
                let price_type = PriceType::parse(payload)?;
                use PriceType::*;
                match price_type {
                    // Valid PriceType for DiscretionPriceType 
                    // excludes "Q" and "m".
                    MarketMakerPeg(..) || Midpoint(..) => Err(
                        BadElementError::InvalidEnum(
                            price_type.encode() as char, 
                            "DiscretionPriceType".to_string()
                        ).into()
                    ),
                    _ => Ok(price_type.map(Self::DiscretionPriceType))
                }
            },

            11 => Ok(SignedPrice::parse(payload)?
                .map(Self::DiscretionPegOffset)),

            12 => {
                let val = u8::from_be_bytes(payload);
                match val {
                    b'P' => Ok(true.map(Self::PostOnly)),
                    b'N' => Ok(false.map(Self::PostOnly)),

                    _ => Err(BadElementError::InvalidEnum(
                            val as char, 
                            "PostOnly".to_string()
                        ).into())
                }
            },

            13 => Ok(u32_from_be_bytes(payload)?
                .map(Self::RandomReserves)),

            14 => Ok(Route::parse(payload)?
                .map(Self::Route)),

            15 => Ok(u32_from_be_bytes(payload)? // TODO type
                .map(Self::ExpireTime)),

            16 => {
                let val = u8::from_be_bytes(payload);
                match val {
                    b'Y' => Ok(true.map(Self::TradeNow)),
                    b'N' => Ok(false.map(Self::TradeNow)),

                    _ => Err(BadElementError::InvalidEnum(
                            val as char, 
                            "TradeNow".to_string()
                        ).into())
                }
            },

            17 => Ok(HandleInst::parse(payload)?
                .map(Self::HandleInst)),

            18 => Ok(BboWeightIndicator::parse(payload)?
                .map(Self::BboWeightIndicator)),

            22 => Ok(u32_from_be_bytes(payload)?
                .map(Self::DisplayQuantity)),

            23 => Ok(Price::parse(payload)?
                .map(Self::DisplayPrice)),

            24 => Ok(u16_from_be_bytes(payload)?
                .map(Self::GroupId)),

            25 => {
                let val = u8::from_be_bytes(payload);
                match val {
                    b'Y' => Ok(true.map(Self::SharesLocated)),
                    b'N' => Ok(false.map(Self::SharesLocated)),

                    _ => Err(BadElementError::InvalidEnum(
                            val as char, 
                            "SharesLocated".to_string()
                        ).into())
                }
            },

            26 => Ok(Broker::parse(payload)? 
                .map(Self::LocateBroker)),

            27 => Ok(Side::parse(payload)?
                .map(Self::Side)),

            28 => Ok(u8::from_be_bytes(payload)?
                .map(Self::UserRefIndex)),


            _ => Err(BadElementError::InvalidEnum(
                option_tag as char, 
                "TagValue".to_string()
            ).into()),
        }
    }

}
