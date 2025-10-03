
mod enter;
mod cancel;
mod replace;
mod modify;
mod permission;
mod query;

use std::fmt;

pub use self::{ 
    enter::EnterOrder, 
    cancel::{ CancelOrder, MassCancel }, 
    replace::ReplaceOrder,
    modify::ModifyOrder,
    permission::{ EnableOrderEntry, DisableOrderEntry },
    query::AccountQuery,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OuchRequest {
    EnterOrder(EnterOrder),
    CancelOrder(CancelOrder),
    ReplaceOrder(ReplaceOrder),
    ModifyOrder(ModifyOrder),
    MassCancel(MassCancel),
    DisableOrderEntry(DisableOrderEntry),
    EnableOrderEntry(EnableOrderEntry),
    AccountQuery(AccountQuery),
}

impl OuchRequest {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            OuchRequest::EnterOrder(msg) => msg.encode(),
            OuchRequest::CancelOrder(msg) => msg.encode(),
            OuchRequest::ReplaceOrder(msg) => msg.encode(),
            OuchRequest::ModifyOrder(msg) => msg.encode(),
            OuchRequest::MassCancel(msg) => msg.encode(),
            OuchRequest::DisableOrderEntry(msg) => msg.encode(),
            OuchRequest::EnableOrderEntry(msg) => msg.encode(),
            OuchRequest::AccountQuery(msg) => msg.encode(),
        }
    }
}

impl fmt::Display for OuchRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            OuchRequest::EnterOrder(..) => "EnterOrder",
            OuchRequest::CancelOrder(..) => "CancelOrder",
            OuchRequest::ReplaceOrder(..) => "ReplaceOrder",
            OuchRequest::ModifyOrder(..) => "ModifyOrder",
            OuchRequest::MassCancel(..) => "MassCancel",
            OuchRequest::DisableOrderEntry(..) => "DisableOrderEntry",
            OuchRequest::EnableOrderEntry(..) => "EnableOrderEntry",
            OuchRequest::AccountQuery(..) => "AccountQuery",
        };

        string.to_string().fmt(f)
    }
}


/// Create an AccountQuery message.
/// ```
/// use slouch::account_query;
/// let request1 = account_query!();
///
/// use slouch::msg::{ OuchRequest, AccountQuery };
/// let request2 = OuchRequest::AccountQuery(AccountQuery::new());
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! account_query {
    () => {
        $crate::msg::OuchRequest::AccountQuery($crate::msg::AccountQuery::new())
    }
}

/// Create a DisableOrderEntry request message.
/// ```
/// use slouch::{ 
///     disable_entry,
///     types::{ UserRefNum, FirmId },
/// };
///
/// let request1 = disable_entry!{
///     user_ref_num: UserRefNum::new(),
///     firm: FirmId::new("FIRM").unwrap(),
/// };
///
/// use slouch::msg::{ OuchRequest, DisableOrderEntry };
///
/// let request2 = OuchRequest::DisableOrderEntry(
///     DisableOrderEntry::new(UserRefNum::new(), FirmId::new("FIRM").unwrap())
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! disable_entry {
    (user_ref_num: $f1:expr, firm: $f2:expr $(,)?) => {
        $crate::msg::OuchRequest::DisableOrderEntry(
            $crate::msg::DisableOrderEntry::new($f1, $f2)
        )
    }
}

/// Create an EnableOrderEntry request message.
/// ```
/// use slouch::{ 
///     enable_entry,
///     types::{ UserRefNum, FirmId },
/// };
///
/// let request1 = enable_entry!{
///     user_ref_num: UserRefNum::new(),
///     firm: FirmId::new("FIRM").unwrap(),
/// };
///
/// use slouch::msg::{ OuchRequest, EnableOrderEntry };
///
/// let request2 = OuchRequest::EnableOrderEntry(
///     EnableOrderEntry::new(UserRefNum::new(), FirmId::new("FIRM").unwrap())
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! enable_entry {
    (user_ref_num: $f1:expr, firm: $f2:expr $(,)?) => {
        $crate::msg::OuchRequest::EnableOrderEntry(
            $crate::msg::EnableOrderEntry::new($f1, $f2)
        )
    }
}

/// Create a MassCancel request message.
/// ```
/// use slouch::{
///     mass_cancel,
///     types::{ UserRefNum, FirmId, StockSymbol },
/// };
///
/// let request1 = mass_cancel!{
///     user_ref_num: UserRefNum::new(),
///     firm: FirmId::new("FIRM").unwrap(),
///     symbol: StockSymbol::new("STONKS").unwrap(),
/// };
///
/// use slouch::msg::{ OuchRequest, MassCancel };
///
/// let request2 = OuchRequest::MassCancel(MassCancel::new(
///     UserRefNum::new(), 
///     FirmId::new("FIRM").unwrap(), 
///     StockSymbol::new("STONKS").unwrap()
/// ));
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! mass_cancel {
    (user_ref_num: $f1:expr, firm: $f2:expr, symbol: $f3:expr $(,)?) => {
        $crate::msg::OuchRequest::MassCancel(
            $crate::msg::MassCancel::new($f1, $f2, $f3)
        )
    }
}

/// Create a CancelOrder request message.
/// WARN: PANIC! This constructor will PANIC if quantity >= 1,000,000.
/// ```
/// use slouch::{ cancel, types::UserRefNum };
///
/// let request1 = cancel!{
///     user_ref_num: UserRefNum::new(),
///     quantity: 0u32,
/// };
///
/// use slouch::msg::{ OuchRequest, CancelOrder };
///
/// let request2 = OuchRequest::CancelOrder(
///     CancelOrder::new(UserRefNum::new(), 0u32).unwrap()
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! cancel {
    (user_ref_num: $f1:expr, quantity: $f2:expr $(,)?) => {
        $crate::msg::OuchRequest::CancelOrder(
            $crate::msg::CancelOrder::assert_new($f1, $f2)
        )
    }
}

/// Create an EnterOrder request message.
/// WARN: PANIC! This constructor will PANIC if quantity >= 1,000,000.
/// ```
/// use slouch::{ 
///     enter, 
///     types::{ 
///         UserRefNum,
///         Side,
///         StockSymbol,
///         Price,
///         TimeInForce,
///         Display,
///         Capacity,
///         CrossType,
///         OrderToken
///     }
/// };
///
/// let request1 = enter!{
///     user_ref_num: UserRefNum::new(),
///     side: Side::Buy,
///     quantity: 0u32,
///     symbol: StockSymbol::new("STONKS").unwrap(),
///     price: Price::new(3, 5001).unwrap(),
///     time_in_force: TimeInForce::Day,
///     display: Display::Visible,
///     capacity: Capacity::Agency,
///     intermarket_sweep_eligibility: false,
///     cross_type: CrossType::Opening,
///     order_token: OrderToken::new("OrderToken").unwrap()
/// };
///
/// use slouch::msg::{ OuchRequest, EnterOrder };
///
/// let request2 = OuchRequest::EnterOrder(
///     EnterOrder::new(
///         UserRefNum::new(), 
///         Side::Buy,
///         0u32,
///         StockSymbol::new("STONKS").unwrap(),
///         Price::new(3, 5001).unwrap(),
///         TimeInForce::Day,
///         Display::Visible,
///         Capacity::Agency,
///         false,
///         CrossType::Opening,
///         OrderToken::new("OrderToken").unwrap()
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
        intermarket_sweep_eligibility: $f9:expr,
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

/// Create a ReplaceOrder request message.
/// WARN: PANIC! This constructor will PANIC if quantity >= 1,000,000.
/// ```
/// use slouch::{ 
///     replace, 
///     types::{ 
///         UserRefNum,
///         Side,
///         StockSymbol,
///         Price,
///         TimeInForce,
///         Display,
///         Capacity,
///         CrossType,
///         OrderToken
///     }
/// };
///
/// let request1 = replace!{
///     user_ref_num: UserRefNum::new(),
///     quantity: 0u32,
///     price: Price::new(3, 5001).unwrap(),
///     time_in_force: TimeInForce::Day,
///     display: Display::Visible,
///     intermarket_sweep_eligibility: false,
///     order_token: OrderToken::new("OrderToken").unwrap()
/// };
///
/// use slouch::msg::{ OuchRequest, ReplaceOrder };
///
/// let request2 = OuchRequest::ReplaceOrder(
///     ReplaceOrder::new(
///         UserRefNum::new(), 
///         0u32,
///         Price::new(3, 5001).unwrap(),
///         TimeInForce::Day,
///         Display::Visible,
///         false,
///         OrderToken::new("OrderToken").unwrap()
///     ).unwrap()
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! replace {
    (
        user_ref_num: $f1:expr,
        quantity: $f2:expr,
        price: $f3:expr,
        time_in_force: $f4:expr,
        display: $f5:expr,
        intermarket_sweep_eligibility: $f6:expr,
        order_token: $f7:expr $(,)?
    ) => {
        $crate::msg::OuchRequest::ReplaceOrder(
            $crate::msg::ReplaceOrder::assert_new(
                $f1, $f2, $f3, $f4, $f5, $f6, $f7
            )
        )
    };
}

/// Create a ModifyOrder request message.
/// WARN: PANIC! This constructor will PANIC if quantity >= 1,000,000.
/// ```
/// use slouch::{ 
///     modify, 
///     types::{ UserRefNum, Side },
/// };
///
/// let request1 = modify!{
///     user_ref_num: UserRefNum::new(),
///     side: Side::Buy,
///     quantity: 0u32,
/// };
///
/// use slouch::msg::{ OuchRequest, ModifyOrder };
///
/// let request2 = OuchRequest::ModifyOrder(
///     ModifyOrder::new(UserRefNum::new(), Side::Buy, 0u32).unwrap()
/// );
///
/// assert_eq!(request1, request2);
/// ```
#[macro_export]
macro_rules! modify {
    (user_ref_num: $f1:expr, side: $f2:expr, quantity: $f3:expr $(,)?) => {
        $crate::msg::OuchRequest::ModifyOrder(
            $crate::msg::ModifyOrder::assert_new($f1, $f2, $f3)
        )
    };
}

