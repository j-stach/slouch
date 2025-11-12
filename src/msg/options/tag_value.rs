
use nom::number::streaming::{ be_u8, be_u16, be_u32, be_u64 };

use nsdq_util::{
    parse_bool,
    encode_bool,
    parse_bool_with_chars,
    encode_bool_with_chars,
    parse_ternary,
    parse_ternary_with_chars,
    encode_ternary,
    encode_ternary_with_chars,
};

use crate::types::{
    Mpid,
    Ternary,
    Price,
    SignedPrice,
    PriceType,
    HandleInst,
    BboWeight,
    RouteId,
    ElapsedTime,
    Side,
};

macro_rules! tag_values {
    (
        $(
            [$tag:expr] $name:ident: $typ:ident 
                $($doc:expr)?;
            { 
                $parser:expr,
                $encoder:expr
            }
        ),* $(,)?
    ) =>{

        /// An optional field on a message is communicated via TagValue.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TagValue {
            $(
                $(#[doc = $doc])?
                $name($typ),
            )*
        }

        impl TagValue {

            pub(crate) fn encode(&self) -> Vec<u8> {

                let (option_tag, encoded_value) = match self {
                    $(
                        Self::$name(val) => ($tag, $encoder(val).to_vec()),
                    )*
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

            // Length-marking byte of tag is not added to `data` when 
            // `OptionalAppendage::parse` calls this method.
            pub(crate) fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {

                let (input, opt) = nom::bytes::complete::take(1usize)(input)?;
                let result = match opt[0] {
                    $( 
                        $tag => {
                            let (input, var) = $parser(input)?;
                            (input, Self::$name(var))
                        },
                    )* 
                    _ => return Err(
                        nom::Err::Error(nom::error::Error::new(
                            input, 
                            nom::error::ErrorKind::Tag
                        ))
                    ),
                };

                Ok(result)
            }

        }

    }
}

tag_values!{

    [1u8] SecondaryOrdRefNum: u64
    "An alternative order reference number used when publishing the order \
    on the NASDAQ market data feeds \
    (identifying, for example, the displayed portion of a reserve order).";
        { be_u64, |i: &u64| u64::to_be_bytes(*i) },

    [2u8] Firm: Mpid "Identifier for the firm placing the order.";
        { Mpid::parse, Mpid::encode },

    [3u8] MinQty: u32 "Must be a round lot.";
        { be_u32, |i: &u32| u32::to_be_bytes(*i) },

    [4u8] Retail: Ternary "Customer Type (Retail/Institutional)";
        { 
            retail_parse, 
            |v: &Option<bool>| encode_ternary_with_chars('R', 'X', ' ', *v) 
        },

    [5u8] MaxFloor: u32 
    "Represents the portion of your order that you wish to have displayed.";
        { be_u32, |i: &u32| u32::to_be_bytes(*i) },

    [6u8] PriceType: PriceType
    "Specifies the type of pricing for the order (e.g., limit, market).";
        { PriceType::parse, PriceType::encode },

    [7u8] PegOffset: SignedPrice "Offset amount for the pegged value.";
        { SignedPrice::parse, SignedPrice::encode },

    [9u8] DiscretionPrice: Price "Price for discretionary order execution.";
        { Price::parse, Price::encode },

    [10u8] DiscretionPriceType: PriceType
    "Limited use of `PriceType`: cant use `MarketMakerPeg` or `Midpoint`.";
        { discretion_price_type_parse, PriceType::encode },

    [11u8] DiscretionPegOffset: SignedPrice 
    "Offset amount for the pegged Discretionary Price.";
        { SignedPrice::parse, SignedPrice::encode },

    [12u8] PostOnly: bool
    "Indicates if the order is post-only (will not execute immediately).";
        { 
            post_only_parse,
            |v: &bool| encode_bool_with_chars('P', 'N', *v) 
        },

    [13u8] RandomReserves: u32 "Shares to do random reserve with.";
        { be_u32, |i: &u32| u32::to_be_bytes(*i) },

    [14u8] Route: RouteId
    "Specifies the routing destination for the order.";
        { RouteId::parse, RouteId::encode },

    [15u8] ExpireTime: ElapsedTime
    "Used to set the duration for Good 'Til Time orders.";
        { ElapsedTime::parse, ElapsedTime::encode },

    [16u8] TradeNow: Ternary
    "Indicates if the order should be executed immediately.";
        { parse_ternary, |v: &Option<bool>| encode_ternary(*v) },

    [17u8] HandleInst: HandleInst "Handling instructions for the order.";
        { HandleInst::parse, HandleInst::encode },

    [18u8] BboWeight: BboWeight 
    "Indicates the weighting of the order in the Best Bid and Offer (BBO).";
        { BboWeight::parse, BboWeight::encode },

    [22u8] DisplayQuantity: u32 
    "Used in the Order Restated Message only. \n \
    Represents an update of an order’s displayed quantity \
    (i.e. an order with reserves).";
        { be_u32, |i: &u32| u32::to_be_bytes(*i) },

    [23u8] DisplayPrice: Price 
    "Used in the Order Restated Message only. \n \
    Represents an update of an order’s displayed price.";
        { Price::parse, Price::encode },

    [24u8] GroupId: u16 
    "Used in the Order Restated Message only. \n \
    Represents an update of an order’s displayed quantity \
    (i.e. an order with reserves).";
        { be_u16, |i: &u16| u16::to_be_bytes(*i) },

    [25u8] SharesLocated: bool "Shares located for short sell.";
        { parse_bool, |v: &bool| encode_bool(*v) },

    [26u8] LocateBroker: Mpid 
    "Broker code from which the locate has been acquired \
    for short sale orders";
        { Mpid::parse, Mpid::encode },

    [27u8] Side: Side "Specifies the side of the order (e.g., buy or sell).";
        { Side::parse, Side::encode },

    [28u8] UserRefIndex: u8 
    "Used in the Order Restated Message only. \n \
    Represents an update of an order’s displayed quantity \
    (i.e. an order with reserves).";
        { be_u8, |i: &u8| u8::to_be_bytes(*i) },

}

fn post_only_parse(input: &[u8]) -> nom::IResult<&[u8], bool> {
   parse_bool_with_chars('P', 'N', input)
}

fn discretion_price_type_parse(input: &[u8]) -> nom::IResult<&[u8], PriceType> {

    let (input, price_type) = PriceType::parse(input)?;

    use PriceType::*;
    match price_type {
        // Valid PriceType for DiscretionPriceType 
        // excludes "Q" and "m".
        MarketMakerPeg | Midpoint => Err(
            nom::Err::Error(nom::error::Error::new(
                input, 
                nom::error::ErrorKind::Tag
            ))
        ),
        _ => Ok((input, price_type))
    }

}

fn retail_parse(input: &[u8]) -> nom::IResult<&[u8], Option<bool>> {
    parse_ternary_with_chars('R', 'X', ' ', input)
}
