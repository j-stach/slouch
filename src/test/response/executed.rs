
use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use nsdq_util::types::time::Timelike;


#[test] fn parse_executed() {

    let mut data: Vec<u8> = vec![b'E'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // u32 for quantity
    data.extend(0u32.to_be_bytes());
    // Price
    data.extend(35001u64.to_be_bytes());
    // Liquidity flag
    data.push(b'A');
    // match number
    data.extend(1u64.to_be_bytes());
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

    let exec = match response {
        OuchResponse::OrderExecuted(exec) => exec,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(exec.timestamp().nanosecond(), 1u32);
    assert_eq!(exec.user_ref_num().val(), 1u32);
    assert_eq!(exec.quantity(), 0u32);
    assert_eq!(exec.price().parts(), (3u64, 5001u64));
    assert_eq!(exec.match_number(), 1u64);
    assert_eq!(exec.liquidity(), Liquidity::Added);

    let options = exec.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

