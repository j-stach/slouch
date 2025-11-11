
use nom::number::streaming::be_u64;

use crate::types::{
    UserRefNum,
    NaiveTime,
    Display,
    RestateReason,
    Price, 
    SignedPrice,
};

crate::msg::define_msg!{

    OrderPriorityUpdate: 
    "Sent whenever priority of the order has been changed by the system.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum 
            { UserRefNum::parse, UserRefNum::encode },
        price: Price 
            { Price::parse, Price::encode },
        display: Display
            { Display::parse, Display::encode },
        order_ref_num: u64
            { be_u64, |i: &u64| u64::to_be_bytes(*i) },
}

crate::msg::define_msg!{

    OrderRestated: 
    "Sent when the system modifies an order as part of its order management.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum 
            { UserRefNum::parse, UserRefNum::encode },
        price: Price 
            { Price::parse, Price::encode },
        reason: RestateReason
            { RestateReason::parse, RestateReason::encode },
}

