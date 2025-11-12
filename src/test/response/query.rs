
use crate::msg::{ OuchResponse, TagValue };

use nsdq_util::types::time::Timelike;


#[test] fn parse_query_response() {

    let mut data: Vec<u8> = vec![b'Q'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
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

    let aqr = match response {
        OuchResponse::AccountQueryResponse(aqr) => aqr,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(aqr.timestamp().nanosecond(), 1u32);
    assert_eq!(aqr.next_user_ref_num().val(), 1u32);

    let options = aqr.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

