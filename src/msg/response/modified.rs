
use nom::number::streaming::be_u32;

use crate::types::{ UserRefNum, Side, NaiveTime };

crate::msg::define_msg!{

    OrderModified: "Sent in response to a ModifyOrder request.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        side: Side
            { Side::parse, Side::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
}

