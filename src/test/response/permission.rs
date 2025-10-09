
use crate::msg::{ OuchResponse, TagValue };

use chrono::Timelike;


#[test] fn parse_enabled() {

    let mut data: Vec<u8> = vec![b'K'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // u32 for quantity
    data.extend(b"FIRM");
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

    let doer = match response {
        OuchResponse::EnableOrderEntryResponse(doer) => doer,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(doer.timestamp().nanosecond(), 1u32);
    assert_eq!(doer.user_ref_num().val(), 1u32);
    assert_eq!(doer.firm().to_string(), String::from("FIRM"));

    let options = doer.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

#[test] fn parse_disabled() {

    let mut data: Vec<u8> = vec![b'G'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // u32 for quantity
    data.extend(b"FIRM");
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

    let doer = match response {
        OuchResponse::DisableOrderEntryResponse(doer) => doer,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(doer.timestamp().nanosecond(), 1u32);
    assert_eq!(doer.user_ref_num().val(), 1u32);
    assert_eq!(doer.firm().to_string(), String::from("FIRM"));

    let options = doer.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

