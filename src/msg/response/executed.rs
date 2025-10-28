
use nom::number::streaming::{ be_u32, be_u64 };
use nsdq_util::NaiveTime;
use crate::types::{ 
    UserRefNum, 
    Price64, 
    LiquidityFlag 
};

crate::msg::define_msg!{

    OrderExecuted: 
    "Informs you that part or all of an order has been executed.";
        // timestamp: NaiveTime
        user_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        price: Price64
            { Price64::parse, Price64::encode },
        liquidity_flag: LiquidityFlag
            { LiquidityFlag::parse, LiquidityFlag::encode },
        match_number: u64
            { be_u64, |i: &u64| u64::to_be_bytes(*i) },
}

