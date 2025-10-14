
use crate::{ 
    modify, 
    msg::{ TagValue, OuchRequest },
};

use crate::types::{ 
    UserRefNum, 
    Side,
};

#[test] fn new() {

    // Macros are tested in the doc comments 
    let mut request = modify!{
        user_ref_num: UserRefNum::new(),
        side: Side::Buy,
        quantity: 0u32
    };
    
    let mo = match request {
        OuchRequest::ModifyOrder(ref mo) => mo,
        _ => panic!{"Damn son, where'd you find that"}
    };
    assert_eq!(mo.user_ref_num(), UserRefNum::new());
    assert_eq!(mo.side(), Side::Buy);
    assert_eq!(mo.quantity(), 0u32);
    assert!(request.options().is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    assert!(!request.options().is_empty());
}

#[test]
#[should_panic]
fn bad_quantity() {
    let _ = modify!{
        user_ref_num: UserRefNum::new(),
        side: Side::Buy,
        quantity: 1_000_000u32
    };
}

#[test] fn encode() {

    let mut request = modify!{
        user_ref_num: UserRefNum::new(),
        side: Side::Buy,
        quantity: 0u32
    };
    
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'M'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include side flag
    should_be.push(b'B');
    // u32 for quantity
    should_be.extend(0u32.to_be_bytes());
    assert_eq!(bytes, should_be);


    request.add_option(TagValue::UserRefIndex(1u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'M'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include side flag
    should_be.push(b'B');
    // u32 for quantity
    should_be.extend(0u32.to_be_bytes());
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(28u8);
    // Include the UserRefIndex byte 
    should_be.push(1u8);
    assert_eq!(bytes, should_be);
}


