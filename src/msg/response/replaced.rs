
use nom::number::streaming::{ be_u32, be_u64 };

use crate::types::{ 
    NaiveTime,
    StockSymbol,
    UserRefNum,
    Side,
    Price,
    TimeInForce,
    Display,
    Capacity,
    CrossType,
    OrderState,
    OrderToken
};

crate::msg::define_msg!{

    OrderReplaced:
    "Acknowledges the receipt and acceptance of a valid ReplaceOrder request. \
    The data fields from the request are echoed back in this message. \n \
    `OrderState::Dead` indicates that a replace was automatically canceled, \
    and no further messages will be received for the order. \n\n\
    NOTE: Accepted values may differ from the entered values for some fields.\
    The accepted price will always be better than or equal to the entered. \
    The accepted Time in Force will always be equal to or shorter in scope \
    than the entered Time in Force.";
        timestamp: NaiveTime { 
            nsdq_util::parse_ouch_time_bold, 
            |v: &NaiveTime| nsdq_util::encode_ouch_time(*v) 
        },
        old_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        new_ref_num: UserRefNum
            { UserRefNum::parse, UserRefNum::encode },
        side: Side
            { Side::parse, Side::encode },
        quantity: u32
            { be_u32, |i: &u32| u32::to_be_bytes(*i) },
        symbol: StockSymbol
            { StockSymbol::parse, StockSymbol::encode },
        price: Price
            { Price::parse, Price::encode },
        time_in_force: TimeInForce
            { TimeInForce::parse, TimeInForce::encode },
        display: Display
            { Display::parse, Display::encode },
        order_ref_num: u64
            { be_u64, |i: &u64| u64::to_be_bytes(*i) },
        capacity: Capacity
            { Capacity::parse, Capacity::encode },
        intermarket_sweep: bool
            { nsdq_util::parse_bool, |v: &bool| nsdq_util::encode_bool(*v) },
        cross_type: CrossType
            { CrossType::parse, CrossType::encode },
        order_state: OrderState
            { OrderState::parse, OrderState::encode },
        order_token: OrderToken
            { OrderToken::parse, OrderToken::encode },
}

