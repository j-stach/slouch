
use nom::number::streaming::{ be_u32, be_u64 };

use crate::error::BadElementError;
use crate::{ types::*, msg::define_msg };


/// Create an EnterOrder request message.
/// WARN: PANIC! This constructor will PANIC if quantity >= 1,000,000.
/// ```
/// use slouch::{ 
///     enter, 
///     types::*,
/// };
///
/// let request1 = enter!{
///     user_ref_num: UserRefNum::new(),
///     side: Side::Buy,
///     quantity: 69u32,
///     symbol: StockSymbol::from("STONKS").unwrap(),
///     price: Price::new(3, 5001).unwrap(),
///     time_in_force: TimeInForce::Day,
///     display: Display::Visible,
///     capacity: Capacity::Agency,
///     intermarket_sweep: false,
///     cross_type: CrossType::Opening,
///     order_token: OrderToken::from("2 th3 M00N").unwrap()
/// };
///
/// use slouch::msg::{ OuchRequest, EnterOrder };
///
/// let request2 = OuchRequest::EnterOrder(
///     EnterOrder::new(
///         UserRefNum::new(), 
///         Side::Buy,
///         69u32,
///         StockSymbol::from("STONKS").unwrap(),
///         Price::new(3, 5001).unwrap(),
///         TimeInForce::Day,
///         Display::Visible,
///         Capacity::Agency,
///         false,
///         CrossType::Opening,
///         OrderToken::from("2 th3 M00N").unwrap()
///     ).unwrap()
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! enter {
    (
        user_ref_num: $f1:expr,
        side: $f2:expr,
        quantity: $f3:expr,
        symbol: $f4:expr,
        price: $f5:expr,
        time_in_force: $f6:expr,
        display: $f7:expr,
        capacity: $f8:expr,
        intermarket_sweep: $f9:expr,
        cross_type: $f10:expr,
        order_token: $f11:expr $(,)?
    ) => {
        $crate::msg::OuchRequest::EnterOrder(
            $crate::msg::EnterOrder::assert_new(
                $f1, $f2, $f3, $f4, $f5, $f6, $f7, $f8, $f9, $f10, $f11
            )
        )
    };
}

define_msg!{
    EnterOrder:
    "Enter a new order. \n\
    For `quantity`, entering over 1,000,000 (maximum shares per order) \
    results in an error.";
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        side: Side
            { Side::parse, Side::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        symbol: StockSymbol
            { StockSymbol::parse, StockSymbol::encode },
        price: Price
            { Price::parse, Price::encode },
        time_in_force: TimeInForce
            { TimeInForce::parse, TimeInForce::encode },
        display: Display
            { Display::parse, Display::encode },
        capacity: Capacity
            { Capacity::parse, Capacity::encode },
        intermarket_sweep: bool
            { nsdq_util::parse_bool, |v: &bool| nsdq_util::encode_bool(*v) },
        cross_type: CrossType
            { CrossType::parse, CrossType::encode },
        order_token: OrderToken
            { OrderToken::parse, OrderToken::encode },
}

impl EnterOrder {

    /// Create a new Enter order.
    /// For `quantity`, entering over 1,000,000 (maximum shares per order) 
    /// results in an error.
    pub fn new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
        symbol: StockSymbol,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        capacity: Capacity,
        intermarket_sweep: bool,
        cross_type: CrossType,
        order_token: OrderToken,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            side,
            quantity,
            symbol,
            price,
            time_in_force,
            display,
            capacity,
            intermarket_sweep,
            cross_type,
            order_token,
            //optional_appendage: OptionalAppendage::new()
        })
    }

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
        symbol: StockSymbol,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        capacity: Capacity,
        intermarket_sweep: bool,
        cross_type: CrossType,
        order_token: OrderToken,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(
            user_ref_num,
            side,
            quantity,
            symbol,
            price,
            time_in_force,
            display,
            capacity,
            intermarket_sweep,
            cross_type,
            order_token
        ).expect("Quantity is acceptable value")
    }

    /*
    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
    /// - Firm
    /// - MinQty
    /// - CustomerType
    /// - MaxFloor
    /// - PriceType
    /// - PegOffset
    /// - DiscretionPrice
    /// - DiscretionPriceType
    /// - DiscretionPegOffset
    /// - PostOnly
    /// - RandomReserves
    /// - ExpireTime
    /// - TradeNow
    /// - HandleInst
    /// - GroupId
    /// - SharesLocated
    /// - LocateBroker
    /// - UserRefIndex
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        use TagValue::*;
        match option {
            Firm(..) |
            MinQty(..) |
            CustomerType(..) |
            MaxFloor(..) |
            PriceType(..) |
            PegOffset(..) |
            DiscretionPrice(..) |
            DiscretionPriceType(..) |
            DiscretionPegOffset(..) |
            PostOnly(..) |
            RandomReserves(..) |
            ExpireTime(..) |
            TradeNow(..) |
            HandleInst(..) |
            GroupId(..) |
            SharesLocated(..) |
            LocateBroker(..) |
            UserRefIndex(..) 
                => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "EnterOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
}

