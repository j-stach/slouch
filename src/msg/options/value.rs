
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
pub enum OptionValue {

    // TODO: TBD: Send email to ask what type this is
    SecondaryOrdRefNum(OrdRefNum),

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

    ///
    PegOffset(SignedPrice),

    ///
    DiscretionPrice(Price),

    /// 
    // TODO: Limit the enum, cant use "Q"
    DiscretionPriceType(PriceType),

    ///
    DiscretionPegOffset(SignedPrice),

    ///
    PostOnly(bool),

    /// Shares to do random reserve with.
    RandomReserves(u32),

    // TODO: TBD
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
    // TODO: TBD
    LocateBroker(Broker),

    /// 
    Side(Side),

    /// User Reference Index - identifies the channel within the given port.
    UserRefIndex(u8),

}

impl OptionValue {

    pub(crate) fn encode(&self) -> Vec<u8> {

        // TODO: Ask about missing tags? => 8, 19-21
        let (option_tag: u8, encoded_value: Vec<u8>) = match self {
            
            SecondaryOrdRefNum(val)     => (1,  val.encode()),
            Firm(val)                   => (2,  val.encode()),
            MinQty(val)                 => (3,  val.to_be_bytes()),
            CustomerType(val)           => (4,  val.encode()),
            MaxFloor(val)               => (5,  val.to_be_bytes()),
            PriceType(val)              => (6,  val.encode()),
            PegOffset(val)              => (7,  val.encode()),
            DiscretionPrice(val)        => (9,  val.encode()),
            DiscretionPriceType(val)    => (10, val.encode()),
            DiscretionPegOffset(val)    => (11, val.encode()),
            PostOnly(val)               => (12, val.encode()), //TODO
            RandomReserves(val)         => (13, val.to_be_bytes()),
            Route(val)                  => (14, val.encode()),
            ExpireTime(val)             => (15, val.to_be_bytes()), //TODO
            TradeNow(val)               => (16, val.encode()), //TODO
            HandleInst(val)             => (17, val.encode()),
            BboWeightIndicator(val)     => (18, val.encode()),
            DisplayQuantity(val)        => (22, val.to_be_bytes()),
            DisplayPrice(val)           => (23, val.encode()),
            GroupId(val)                => (24, val.to_be_bytes()),
            SharesLocated(val)          => (25, val.encode()), //TODO
            LocateBroker(val)           => (26, val.encode()),
            Side(val)                   => (27, val.encode()),
            UserRefIndex(val)           => (28, val.to_be_bytes()),
        };

        // Start encoded array with length (calculated, not tracked);
        // this is the start of the protocol's `TagValue` type.
        // Safely assume that length will be less than u8::MAX,
        // because each enum variant's inner type encodes to less.
        let length_in_bytes: u8 = encoded_value.len() as u8 + 1;

        let mut data = vec![length_in_bytes, option_tag];
        data.extend(encoded_value);

        // This is formatted as the protocol's TagValue, 
        // including length marker:
        data
    }

    pub(crate) fn parse(data: Vec<u8>) -> Result<Self, OuchError> {

        // NOTE: Ignore data[0] as it will contain the protocol TagValue's
        // length-marking byte, which is not needed here.

        let option_tag = u8::from_be_bytes(data[1]);
        let payload = &data[2..];

        match option_tag {

            1 => Ok(SecondaryOrdRefNum::parse(payload)?
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

            10 => Ok(PriceType::parse(payload)? // TODO Exclude "m" and "Q"
                .map(Self::DiscretionPriceType)),

            11 => Ok(SignedPrice::parse(payload)?
                .map(Self::DiscretionPegOffset)),

            12 => Ok(XX::parse(payload)? // TODO
                .map(Self::PostOnly)),

            13 => Ok(u32_from_be_bytes(payload)?
                .map(Self::RandomReserves)),

            14 => Ok(Route::parse(payload)?
                .map(Self::Route)),

            15 => Ok(u32_from_be_bytes(payload)? // TODO type
                .map(Self::ExpireTime)),

            16 => Ok(XX::parse(payload)? // TODO
                .map(Self::TradeNow)),

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

            25 => Ok(XX::parse(payload)? // TODO
                .map(Self::SharesLocated)),

            26 => Ok(Broker::parse(payload)? // TODO type
                .map(Self::LocateBroker)),

            27 => Ok(Side::parse(payload)?
                .map(Self::Side)),

            28 => Ok(u8::from_be_bytes(payload)?
                .map(Self::UserRefIndex)),


            _ => BadElementError::InvalidEnum(
                option_tag as char, 
                "OptionValue".to_string()
            ).into(),

        }

    }
}
