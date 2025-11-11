
use crate::types::{ NaiveTime, UserRefNum };

crate::msg::define_msg!{

    AccountQueryResponse: "Indicates the next available UserRefNum.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        next_user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode }
}

