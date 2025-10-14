
use crate::{ 
    enable_entry, disable_entry, 
    msg::{ TagValue, OuchRequest },
};

use crate::types::{ 
    UserRefNum, 
    FirmId, 
};


#[test] fn new_enable() {

    let mut request = enable_entry!{
        user_ref_num: UserRefNum::new(),
        firm: FirmId::from("FIRM").unwrap(),
    };
    
    let eoe = match request {
        OuchRequest::EnableOrderEntry(ref eoe) => eoe,
        _ => panic!{"Damn son, where'd you find that"}
    };
    assert_eq!(eoe.user_ref_num(), UserRefNum::new());
    assert_eq!(eoe.firm(), FirmId::from("FIRM").unwrap());
    assert!(request.options().is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    assert!(!request.options().is_empty());
}

#[test] fn encode_enable() {

    let mut request = enable_entry!{
        user_ref_num: UserRefNum::new(),
        firm: FirmId::from("FIRM").unwrap(),
    };
    
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'E'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include the firm
    should_be.extend(b"FIRM");
    // Optional appendage
    should_be.extend(0u16.to_be_bytes());
    assert_eq!(bytes, should_be);


    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'E'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include the firm
    should_be.extend(b"FIRM");
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(28u8);
    // Include the UserRefIndex byte 
    should_be.push(0u8);
    assert_eq!(bytes, should_be);
}


#[test] fn new_disable() {

    let mut request = disable_entry!{
        user_ref_num: UserRefNum::new(),
        firm: FirmId::from("FIRM").unwrap(),
    };
    
    let doe = match request {
        OuchRequest::DisableOrderEntry(ref doe) => doe,
        _ => panic!{"Damn son, where'd you find that"}
    };
    assert_eq!(doe.user_ref_num(), UserRefNum::new());
    assert_eq!(doe.firm(), FirmId::from("FIRM").unwrap());
    assert!(request.options().is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    assert!(!request.options().is_empty());
}

#[test] fn encode_disable() {

    let mut request = disable_entry!{
        user_ref_num: UserRefNum::new(),
        firm: FirmId::from("FIRM").unwrap(),
    };
    
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'D'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include the firm
    should_be.extend(b"FIRM");
    // Optional appendage
    should_be.extend(0u16.to_be_bytes());
    assert_eq!(bytes, should_be);


    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'D'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include the firm
    should_be.extend(b"FIRM");
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(28u8);
    // Include the UserRefIndex byte 
    should_be.push(0u8);
    assert_eq!(bytes, should_be);
}

