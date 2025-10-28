
use nom::number::streaming::be_u64;
use nsdq_util::NaiveTime;

use crate::types::{
    UserRefNum,
    Display,
    RestateReason,
    Price64, 
    SignedPrice,
};

crate::msg::define_msg!{

    OrderPriorityUpdate: 
    "Sent whenever priority of the order has been changed by the system.";
        //timestamp: NaiveTime,
        user_ref_num: UserRefNum 
            { UserRefNum::parse, UserRefNum::encode },
        price: Price64 
            { Price64::parse, Price64::encode },
        display: Display
            { Display::parse, Display::encode },
        order_ref_num: u64
            { be_u64, |i: &u64| u64::to_be_bytes(*i) },
}

crate::msg::define_msg!{

    OrderRestated: 
    "Sent when the system modifies an order as part of its order management.";
        //timestamp: NaiveTime,
        user_ref_num: UserRefNum 
            { UserRefNum::parse, UserRefNum::encode },
        price: Price64 
            { Price64::parse, Price64::encode },
        reason: RestateReason
            { RestateReason::parse, RestateReason::encode },
}

