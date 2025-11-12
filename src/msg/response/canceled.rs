
use nom::number::streaming::be_u32;

use crate::types::{ 
    NaiveTime, 
    Mpid, 
    StockSymbol, 
    UserRefNum,
    Price,
    CancelReason,
    AiqStrategy,
    Liquidity,
};


crate::msg::define_msg!{
    
    OrderCanceled: 
    "Informs you that an order has been reduced or canceled. \n \
    OrderCanceled does not necessarily mean the entire order is dead; \
    some portion of the order may still be alive.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        quantity: u32 
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        reason: CancelReason
            { CancelReason::parse, CancelReason::encode },
}

crate::msg::define_msg!{

    AiqCanceled:
    "";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        decrement_shares: u32 
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        reason: CancelReason
            { CancelReason::parse, CancelReason::encode },
        quantity_prevented: u32 
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        price: Price
            { Price::parse, Price::encode },
        liquidity: Liquidity
            { Liquidity::parse, Liquidity::encode },
        aiq_strategy: AiqStrategy
            { AiqStrategy::parse, AiqStrategy::encode },
}

crate::msg::define_msg!{

    CancelPending:
    "Sent in response to a CancelOrder request for a cross order during a \
    pre-cross late period signifying that it cannot be canceled at this time, \
    but any unexecuted portion of this order will automatically be canceled \
    immediately after the cross completes. \n \
    This message will only be sent once for a given UserRefNum. ";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
}

crate::msg::define_msg!{

    CancelRejected:
    "Sent in response to a partial cancel request (with non-zero “quantity”) \
    for a cross order during a pre-cross late period signifying that it \
    cannot be partially canceled at this time. \n \
    No automatic cancel will be scheduled for this order. \
    Clients could repeat their request for any unexecuted portion \
    of the order after the cross completes. \n \
    This message will only be sent once for a given UserRefNum.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
}

crate::msg::define_msg!{

    MassCancelResponse:
    "Acknowledges the receipt of a valid MassCancel request. \
    The data fields from the request are echoed back in this message.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        firm: Mpid
            { Mpid::parse, Mpid::encode },
        symbol: StockSymbol
            { StockSymbol::parse, StockSymbol::encode },
}

