
use nom::number::streaming::{ be_u32, be_u64 };

use crate::types::{ 
    NaiveTime,
    UserRefNum, 
    Price, 
    Liquidity 
};

crate::msg::define_msg!{

    OrderExecuted: 
    "Informs you that part or all of an order has been executed.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        price: Price
            { Price::parse, Price::encode },
        liquidity: Liquidity
            { Liquidity::parse, Liquidity::encode },
        match_number: u64
            { be_u64, |i: &u64| u64::to_be_bytes(*i) },
}

