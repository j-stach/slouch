
// restated
// priorit update

use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use chrono::Timelike;


#[test] fn parse_priority_update() {

    let mut data: Vec<u8> = vec![b'T'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // Price
    data.extend(35001u64.to_be_bytes());
    // Display
    data.push(b'Y');
    //ordrefnum
    data.extend(1u64.to_be_bytes());
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

    let update = match response {
        OuchResponse::OrderPriorityUpdate(update) => update,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(update.timestamp().nanosecond(), 1u32);
    assert_eq!(update.user_ref_num().val(), 1u32);
    assert_eq!(update.price().dollars(), 3u32);
    assert_eq!(update.price().cents(), 5001u16);
    assert_eq!(update.display(), Display::Visible);
    assert_eq!(update.order_ref_num(), 1u64);

    let options = update.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

#[test] fn parse_restate() {

    let mut data: Vec<u8> = vec![b'R'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // reason
    data.push(b'R');
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

    let restate = match response {
        OuchResponse::OrderRestated(restate) => restate,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(restate.timestamp().nanosecond(), 1u32);
    assert_eq!(restate.user_ref_num().val(), 1u32);
    assert_eq!(restate.reason(), RestateReason::Refresh);

    let options = restate.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

