
use nom::number::streaming::be_u32;
use nsdq_util::NaiveTime;

use crate::types::{ UserRefNum, Side };

crate::msg::define_msg!{

    OrderModified: "Sent in response to a ModifyOrder request.";
        // TODO timestamp: NaiveTime,
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        side: Side
            { Side::parse, Side::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
}

