
use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use nsdq_util::types::time::Timelike;


#[test] fn parse_rejected() {

    let mut data: Vec<u8> = vec![b'J'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // Include reject reason
    data.extend(&0x0006_u16.to_be_bytes());
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

    let reject = match response {
        OuchResponse::OrderRejected(reject) => reject,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(reject.timestamp().nanosecond(), 1u32);
    assert_eq!(reject.user_ref_num().val(), 1u32);
    assert_eq!(reject.reason(), RejectReason::FatFinger);
    assert_eq!(reject.order_token().to_string(), String::from("To The Moon"));

    let options = reject.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

