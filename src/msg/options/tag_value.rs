
use nom::number::streaming::{ be_u8, be_u16, be_u32, be_u64 };

use nsdq_util::{
    parse_bool,
    parse_ternary,
};

use crate::types::{
    Mpid,
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
                    _ => { 

                        // TODO: Parse error
                        todo![] 
                    }
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

    // Specifies the type of customer for the order 
    // (e.g., retail, institutional).
    // TODO: encode_ternary/encode_bool
    //[4u8] Retail: Option<bool> "Customer Type (Retail/Institutional)";
    //    { parse_ternary, /*TODO*/ }

    [5u8] MaxFloor: u32 
    "Represents the portion of your order that you wish to have displayed.";
        { be_u32, |i: &u32| u32::to_be_bytes(*i) },

    //Specifies the type of pricing for the order (e.g., limit, market).
    [6u8] PriceType: PriceType;
        { PriceType::parse, PriceType::encode },

    [7u8] PegOffset: SignedPrice "Offset amount for the pegged value.";
        { SignedPrice::parse, SignedPrice::encode },

    [9u8] DiscretionaryPrice: Price 
    "Price for discretionary order execution.";
        { Price::parse, Price::encode },

    // Limited use of `PriceType`: cant use `MarketMakerPeg` or `Midpoint`.
    [10u8] DiscretionPriceType: PriceType;
        { PriceType::parse, PriceType::encode },

    /*
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

    */

    [11u8] DiscretionPegOffset: SignedPrice 
    "Offset amount for the pegged Discretionary Price.";
        { SignedPrice::parse, SignedPrice::encode },

    // Indicates if the order is post-only (will not execute immediately).
    //[12u8] PostOnly: bool;
    //    { |i: &[u8]| parse_bool_with_chars('P', 'N', i), /*TODO*/ }

    [13u8] RandomReserves: u32 "Shares to do random reserve with.";
        { be_u32, |i: &u32| u32::to_be_bytes(*i) },

    // Specifies the routing destination for the order.
    [14u8] Route: RouteId;
        { RouteId::parse, RouteId::encode },

    // Seconds to live. 
    // Must be less than 86400 (number of seconds in a day).
    //[15u8] ExpireTime: TODO ...NaiveTime?;
    //    { RouteId::parse, RouteId::encode },

    // Indicates if the order should be executed immediately.
    //[16u8] TradeNow: bool;
    //    { parse_ternary, /*TODO*/ },

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

    // Indicates if the order is post-only (will not execute immediately).
    //[25u8] SharesLocated: bool;
    //    { parse_bool, /*TODO*/ }

    [26u8] LocateBroker: Mpid 
    "Broker code from which the locate has been acquired \
    for short sale orders";
        { Mpid::parse, Mpid::encode },

    //[27u8] Side: Side "Specifies the side of the order (e.g., buy or sell).";
    //    { Side::parse, Side::encode },

    [28u8] UserRefIndex: u8 
    "Used in the Order Restated Message only. \n \
    Represents an update of an order’s displayed quantity \
    (i.e. an order with reserves).";
        { be_u8, |i: &u8| u8::to_be_bytes(*i) },

}

