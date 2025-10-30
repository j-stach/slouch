
use crate::types::{ NaiveTime, Mpid, UserRefNum };

crate::msg::define_msg!{

    DisableOrderEntryResponse:
    "Acknowledges the receipt of the DisableOrderEntry request. \
    The values from the request message are echoed back in this message.";
        //timestamp: NaiveTime,
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        firm: Mpid 
            { Mpid::parse, Mpid::encode },
}

crate::msg::define_msg!{

    EnableOrderEntryResponse:
    "Acknowledges the receipt of the EnableOrderEntry request. \
    The values from the request message are echoed back in this message.";
        //timestamp: NaiveTime,
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        firm: Mpid 
            { Mpid::parse, Mpid::encode },
}

