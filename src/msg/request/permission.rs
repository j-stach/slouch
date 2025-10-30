
use nom::number::streaming::{ be_u32, be_u64 };
use nsdq_util::{ Mpid, StockSymbol };

use crate::error::BadElementError;
use crate::{ types::*, msg::define_msg };


/// Create a DisableOrderEntry request message.
/// ```
/// use slouch::{ 
///     disable_entry,
///     types::{ Mpid, UserRefNum },
/// };
///
/// let request1 = disable_entry!{
///     user_ref_num: UserRefNum::new(),
///     firm: Mpid::from("FIRM").unwrap(),
/// };
///
/// use slouch::msg::{ OuchRequest, DisableOrderEntry };
///
/// let request2 = OuchRequest::DisableOrderEntry(
///     DisableOrderEntry::new(UserRefNum::new(), Mpid::from("FIRM").unwrap())
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

define_msg!{
    DisableOrderEntry:
    "Prevent a firm from entering new orders on this account.";
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        firm: Mpid
            { Mpid::parse, Mpid::encode },
}

impl DisableOrderEntry {

    /// Create a new Disable Entry request.
    pub fn new(
        user_ref_num: UserRefNum,
        firm: Mpid,
    ) -> Self {
        
        Self {
            user_ref_num,
            firm,
            //optional_appendage: OptionalAppendage::new(),
        }
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
                    "DisableOrderEntry".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
}


/// Create an EnableOrderEntry request message.
/// ```
/// use slouch::{ 
///     enable_entry,
///     types::{ UserRefNum, Mpid },
/// };
///
/// let request1 = enable_entry!{
///     user_ref_num: UserRefNum::new(),
///     firm: Mpid::from("FIRM").unwrap(),
/// };
///
/// use slouch::msg::{ OuchRequest, EnableOrderEntry };
///
/// let request2 = OuchRequest::EnableOrderEntry(
///     EnableOrderEntry::new(UserRefNum::new(), Mpid::from("FIRM").unwrap())
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

define_msg!{
    EnableOrderEntry:
    "Allow a firm to enter new orders on this account.";
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        firm: Mpid
            { Mpid::parse, Mpid::encode },
}

impl EnableOrderEntry {

    /// Create a new Enable Entry request.
    pub fn new(
        user_ref_num: UserRefNum,
        firm: Mpid,
    ) -> Self {
        
        Self {
            user_ref_num,
            firm,
            //optional_appendage: OptionalAppendage::new(),
        }
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
                    "EnableOrderEntry".to_string()
                ))
            },
        }

        Ok(self.optional_appendage.add(option))
    }
*/
}

