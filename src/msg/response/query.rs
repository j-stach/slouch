
use crate::types::{ NaiveTime, UserRefNum };

crate::msg::define_msg!{

    AccountQueryResponse: "Indicates the next available UserRefNum.";
        // TODO timestamp: NaiveTime
        next_user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode }
}

