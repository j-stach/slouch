
use nom::number::streaming::{ be_u32, be_u64 };

use crate::error::BadElementError;
use crate::{ types::*, msg::define_msg };


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

define_msg!{
    CancelOrder:
    "Cancel or reduce shares on an existing order. \n \
    Duplicate cancel requests for the same UserRefNum will be ignored by OUCH. \
    Canceling an order after its execution will be silently ignored.";
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
}

impl CancelOrder {

    /// Create a new Cancel order. 
    ///
    /// `user_ref_num` refers to the order to be canceled.
    ///
    /// `quantity` limits the maximum number of shares that remain to be 
    /// executed after the (partial) cancel is applied. 
    /// Entering over 1,000,000 (maximum shares per order) results in an error.
    /// Entering a value greater than the original quantity does nothing.
    /// Entering `0` will cancel all remaining open shares on the order.
    pub fn new(
        user_ref_num: UserRefNum,
        quantity: u32,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            quantity,
            optional_appendage: crate::msg::options::OptionalAppendage::new(),
        })
    }

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        user_ref_num: UserRefNum,
        quantity: u32,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(user_ref_num, quantity)
            .expect("Quantity is acceptable value")
    }
    
    /*
    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
    /// - UserRefIndex
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        match option {
            TagValue::UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "CancelOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
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
///     firm: Mpid::from("FIRM").unwrap(),
///     symbol: StockSymbol::from("STONKS").unwrap(),
/// };
///
/// use slouch::msg::{ OuchRequest, MassCancel };
///
/// let request2 = OuchRequest::MassCancel(MassCancel::new(
///     UserRefNum::new(), 
///     Mpid::from("FIRM").unwrap(), 
///     StockSymbol::from("STONKS").unwrap()
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

define_msg!{
    MassCancel: 
    "Cancel all active orders for a symbol.";
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        firm: Mpid
            { Mpid::parse, Mpid::encode },
        symbol: StockSymbol
            { StockSymbol::parse, StockSymbol::encode },
}

impl MassCancel {

    /// Create a new Mass Cancel order. 
    pub fn new(
        user_ref_num: UserRefNum,
        firm: Mpid,
        symbol: StockSymbol,
    ) -> Self {

        Self {
            user_ref_num,
            firm,
            symbol,
            optional_appendage: crate::msg::options::OptionalAppendage::new(),
        }
    }

    /*
    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
    /// - GroupId
    /// - UserRefIndex
    /// - Side
    /// For `GroupId`: if the value is set to 0, 
    /// all orders without a Group ID will be canceled.
    /// If the value is not specified, all orders, regardless
    /// of whether they have a Group ID or not, will be canceled.
    pub fn add_option(
        &mut self, 
        option: TagValue
    ) -> Result<(), BadElementError> {

        // Filter out unacceptable TagValue types.
        use TagValue::*;
        match option {
            GroupId(..) |
            Side(..) |
            UserRefIndex(..) 
                => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "MassCancel".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
}


