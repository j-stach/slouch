
use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use chrono::Timelike;


#[test] fn parse_accepted() {

    let mut data: Vec<u8> = vec![b'A'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // Include side flag
    data.push(b'B');
    // u32 for quantity
    data.extend(0u32.to_be_bytes());
    // Symbol
    data.extend(b"STONKS  ");
    // Price
    data.extend(35001u64.to_be_bytes());
    // Time in Force
    data.push(b'0');
    // Display
    data.push(b'Y');
    //ordrefnum
    data.extend(1u64.to_be_bytes());
    // Capacity
    data.push(b'A');
    // ISE
    data.push(b'N');
    // Cross Type
    data.push(b'O');
    //orderstatus
    data.push(b'L');
    // CIOrdId
    data.extend(b"To The Moon   ");
    // Include the appendage length marker
    data.extend(3u16.to_be_bytes());
    // Include the tag value length marker
    data.push(2u8);
    // Include option tag
    data.push(28u8);
    // Include the UserRefIndex byte 
    data.push(0u8);

    let response = OuchResponse::try_from(&data[0..])
        .expect("Should be valid data");

    let accepted = match response {
        OuchResponse::OrderAccepted(accepted) => accepted,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(accepted.timestamp().nanosecond(), 1u32);
    assert_eq!(accepted.user_ref_num().val(), 1u32);
    assert_eq!(accepted.side(), Side::Buy);
    assert_eq!(accepted.quantity(), 0u32);
    assert_eq!(accepted.symbol().to_string(), String::from("STONKS"));
    assert_eq!(accepted.price().dollars(), 3u32);
    assert_eq!(accepted.price().cents(), 5001u16);
    assert_eq!(accepted.time_in_force(), TimeInForce::Day);
    assert_eq!(accepted.display(), Display::Visible);
    assert_eq!(accepted.order_ref_num(), 1u64);
    assert_eq!(accepted.capacity(), Capacity::Agency);
    assert_eq!(accepted.intermarket_sweep_eligibility(), false);
    assert_eq!(accepted.cross_type(), CrossType::Opening);
    assert_eq!(accepted.order_state(), OrderState::Live);
    assert_eq!(accepted.order_token().to_string(), String::from("To The Moon"));

    let options = accepted.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

