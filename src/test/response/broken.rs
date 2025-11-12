
use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use nsdq_util::types::time::Timelike;


#[test] fn parse_broken_trade() {

    let mut data: Vec<u8> = vec![b'B'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // matchnum
    data.extend(1u64.to_be_bytes());
    // brokenreason
    data.push(b'E');
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

    let (_data, response) = OuchResponse::parse(&data[0..])
        .expect("Should be valid data");

    let broken = match response {
        OuchResponse::BrokenTrade(broken) => broken,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(broken.timestamp().nanosecond(), 1u32);
    assert_eq!(broken.user_ref_num().val(), 1u32);
    assert_eq!(broken.match_number(), 1u64);
    assert_eq!(broken.reason(), BrokenReason::Erroneous);
    assert_eq!(broken.order_token().to_string(), String::from("To The Moon"));

    let options = broken.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

