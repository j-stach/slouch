
use nom::number::streaming::{ be_u32, be_u64 };

use crate::error::BadElementError;
use crate::{ types::*, msg::define_msg };


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

define_msg!{
    ModifyOrder: 
    "Modify values for an existing order, without affecting priority.";
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        side: Side
            { Side::parse, Side::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
}

impl ModifyOrder {

    /// Create a new Modify request.
    ///
    /// `user_ref_num` refers to the order to be modified.
    ///
    /// For `quantity`, entering over 1,000,000 (maximum shares per order) 
    /// results in an error.
    ///
    /// The following `side` modifications are allowed:
    /// Sell <-> SellShortExempt
    /// Sell <-> SellShort
    /// SellShortExempt <-> SellShort
    pub fn new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
    ) -> Result<Self, BadElementError> {

        if quantity >= 1_000_000 {
            return Err(BadElementError::InvalidValue("Quantity".to_string()))
        }

        Ok(Self {
            user_ref_num,
            side,
            quantity,
            optional_appendage: crate::msg::options::OptionalAppendage::new(),
        })
    }

    /// WARN: Panics!
    /// This constructor will panic if quantity >= 1,000,000.
    pub fn assert_new(
        user_ref_num: UserRefNum,
        side: Side,
        quantity: u32,
    ) -> Self {

        assert!(quantity < 1_000_000);
        Self::new(user_ref_num, side, quantity)
            .expect("Quantity is acceptable value")
    }

    /*
    /// Add a `TagValue` to the optional appendage.
    /// Available options for this message type are:
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
            SharesLocated(..) |
            LocateBroker(..) |
            UserRefIndex(..) => { /* Continue */ },

            _ => {
                return Err(BadElementError::InvalidOption(
                    "ModifyOrder".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
    */
} 

