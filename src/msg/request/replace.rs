
use nom::number::streaming::{ be_u32, be_u64 };

use crate::error::BadElementError;
use crate::{ types::*, msg::define_msg };


/// Create a ReplaceOrder request message.
/// WARN: PANIC! This constructor will PANIC if quantity >= 1,000,000.
/// ```
/// use slouch::{ 
///     replace, 
///     types::*,
/// };
///
/// let request1 = replace!{
///     old_ref_num: UserRefNum::new(),
///     new_ref_num: UserRefNum::new(),
///     quantity: 420u32,
///     price: Price::new(3, 5001).unwrap(),
///     time_in_force: TimeInForce::Day,
///     display: Display::Visible,
///     intermarket_sweep: false,
///     order_token: OrderToken::from("To The Moon").unwrap()
/// };
///
/// use slouch::msg::{ OuchRequest, ReplaceOrder };
///
/// let request2 = OuchRequest::ReplaceOrder(
///     ReplaceOrder::new(
///         UserRefNum::new(), 
///         UserRefNum::new(), 
///         420u32,
///         Price::new(3, 5001).unwrap(),
///         TimeInForce::Day,
///         Display::Visible,
///         false,
///         OrderToken::from("To The Moon").unwrap()
///     ).unwrap()
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! replace {
    (
        old_ref_num: $f1:expr,
        new_ref_num: $f2:expr,
        quantity: $f3:expr,
        price: $f4:expr,
        time_in_force: $f5:expr,
        display: $f6:expr,
        intermarket_sweep: $f7:expr,
        order_token: $f8:expr $(,)?
    ) => {
        $crate::msg::OuchRequest::ReplaceOrder(
            $crate::msg::ReplaceOrder::assert_new(
                $f1, $f2, $f3, $f4, $f5, $f6, $f7, //$f8
            )
        )
    };
}

define_msg!{
    ReplaceOrder: 
    "Cancel and replace an existing order in a single message.\n\
    If the original order is no longer live or the new UserRefNum is invalid,\
    the replacement will be silently ignored. \n\
    If the original order is live but the details of the replace are invalid, \
    the original order will be canceled but a new one will not be entered. \
    In this case, the new UserRefNum will not be consumed and may be reused.\n\
    If the original order is live but the cannot be canceled \
    (e.g., the existing Order is a cross order in the late period), \
    there will be an OrderReject reponse and the order will not be replaced. \
    The OrderReject consumes the new UserRefNum, so it may not be reused. \n\
    Replacing an order gives it a new timestamp/time priority on the book. \
    See the OUCH 5.0 specification for more information.";
        old_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        new_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        price: Price
            { Price::parse, Price::encode },
        time_in_force: TimeInForce
            { TimeInForce::parse, TimeInForce::encode },
        display: Display
            { Display::parse, Display::encode },
        intermarket_sweep: bool
            { nsdq_util::parse_bool, |v: &bool| nsdq_util::encode_bool(*v) },
        order_token: OrderToken
            { OrderToken::parse, OrderToken::encode },
}

impl ReplaceOrder {
    
    /// Crate a new Replace order.
    ///
    /// The first UserRefNum is that of the existing order to cancel; 
    /// the second must be a valid new UserRefNum for the replacement order.
    ///
    /// If a `TagValue::UserRefIndex` option is used on the original order, 
    /// it must also be added to this request. (See `add_option` below.)
    pub fn new(
        old_ref_num: UserRefNum,
        new_ref_num: UserRefNum,
        quantity: u32,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        intermarket_sweep: bool,
        order_token: OrderToken,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            old_ref_num,
            new_ref_num,
            quantity,
            price,
            time_in_force,
            display,
            intermarket_sweep,
            order_token,
            optional_appendage: crate::msg::options::OptionalAppendage::new(),
        })
    }

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        old_ref_num: UserRefNum,
        new_ref_num: UserRefNum,
        quantity: u32,
        price: Price,
        time_in_force: TimeInForce,
        display: Display,
        intermarket_sweep: bool,
        order_token: OrderToken,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(
            old_ref_num,
            new_ref_num,
            quantity,
            price,
            time_in_force,
            display,
            intermarket_sweep,
            order_token
        ).expect("Quantity is acceptable value")
    }

    /*
    /// Add an optional field to the optional appendage.
    /// The majority of fields from the Enter Order Message are supported 
    /// in this message, except for `Firm` and `GroupId`, which are inherited
    /// from the original order. 
    /// Per spec, `CustomerType` is also not accepted, 
    /// although `Side` may be optionally supplied.
    /// Available options for this message type are:
    /// - MinQty
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
    /// - SharesLocated
    /// - LocateBroker
    /// - UserRefIndex
    /// - Side
    /// NOTE: If a `UserRefIndex` option is used on the original order, 
    /// it MUST also be added here.
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        use TagValue::*;
        match option {
            MinQty(..) |
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
            Side(..) |
            SharesLocated(..) |
            LocateBroker(..) |
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "ReplaceOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
}

