
use nsdq_util::NaiveTime;
use crate::types::UserRefNum;

crate::msg::define_msg!{

    AccountQueryResponse: "Indicates the next available UserRefNum.";
        // TODO timestamp: NaiveTime
        next_user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode }
}

