
use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use chrono::Timelike;


#[test] fn parse_modified() {

    let mut data: Vec<u8> = vec![b'M'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // Include side flag
    data.push(b'B');
    // u32 for quantity
    data.extend(0u32.to_be_bytes());
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

    let modify = match response {
        OuchResponse::OrderModified(modify) => modify,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(modify.timestamp().nanosecond(), 1u32);
    assert_eq!(modify.user_ref_num().val(), 1u32);
    assert_eq!(modify.side(), Side::Buy);
    assert_eq!(modify.quantity(), 0u32);

    let options = modify.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

