
use crate::types::{
    NaiveTime,
    UserRefNum,
    BrokenReason,
    OrderToken,
};

crate::msg::define_msg!{

    BrokenTrade: 
    "Informs you that an execution has been broken. \
    The trade is no longer good and will not clear. \n \
    You will only receive a BrokenTrade after first receiving an \
    OrderExecuted message for a given order.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        reason: BrokenReason
            { BrokenReason::parse, BrokenReason::encode }
}

