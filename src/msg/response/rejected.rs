
use nom::number::streaming::be_u32;
use nsdq_util::{};
use crate::types::{ 
    UserRefNum,
    NaiveTime, 
    Mpid, 
    StockSymbol,
    //RejectReason,
    OrderToken
};

crate::msg::define_msg!{

    OrderRejected:
    "May be sent in response to an EnterOrder or ReplaceOrder request if the \
    order or replace cannot be accepted at this time. \n \
    NOTE: The UserRefNum of a Rejected Message cannot be re-used.";
        //timestamp: NaiveTime,
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        //reason: RejectReason
        //    { RejectReason::parse, RejectReason::encode },
        order_token: OrderToken
            { OrderToken::parse, OrderToken::encode },
}

