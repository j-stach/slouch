
use crate::{ 
    cancel, mass_cancel, 
    msg::{ TagValue, OuchRequest },
};

use crate::types::{ 
    UserRefNum, 
    Mpid, 
    StockSymbol
};

#[test] fn new_cancel() {

    // Macros are tested in the doc comments 
    let mut request = cancel!{
        user_ref_num: UserRefNum::new(),
        quantity: 0u32
    };
    
    let co = match request {
        OuchRequest::CancelOrder(ref co) => co,
        _ => panic!{"Damn son, where'd you find that"}
    };
    assert_eq!(co.user_ref_num(), UserRefNum::new());
    assert_eq!(co.quantity(), 0u32);
    assert!(request.options().is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    assert!(!request.options().is_empty());
}

#[test]
#[should_panic]
fn bad_quantity() {
    let _ = cancel!{
        user_ref_num: UserRefNum::new(),
        quantity: 1_000_000u32
    };
}

#[test] fn encode_cancel() {

    let mut request = cancel!{
        user_ref_num: UserRefNum::new(),
        quantity: 0u32
    };

    let bytes = request.clone().encode();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'X'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // u32 for quantity
    should_be.extend(0u32.to_be_bytes());
    // Optional appendage
    should_be.extend(0u16.to_be_bytes());
    assert_eq!(bytes, should_be);


    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().encode();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'X'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // u32 for quantity
    should_be.extend(0u32.to_be_bytes());
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


#[test] fn new_mass_cancel() {

    let mut request = mass_cancel!{
        user_ref_num: UserRefNum::new(),
        firm: Mpid::from("FIRM").unwrap(),
        symbol: StockSymbol::from("STONKS").unwrap(),
    };
    
    let mc = match request {
        OuchRequest::MassCancel(ref mc) => mc,
        _ => panic!{"Damn son, where'd you find that"}
    };
    assert_eq!(mc.user_ref_num(), UserRefNum::new());
    assert_eq!(mc.firm(), Mpid::from("FIRM").unwrap());
    assert_eq!(mc.symbol(), StockSymbol::from("STONKS").unwrap());
    assert!(request.options().is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    assert!(!request.options().is_empty());
}

#[test] fn encode_mass_cancel() {

    let mut request = mass_cancel!{
        user_ref_num: UserRefNum::new(),
        firm: Mpid::from("FIRM").unwrap(),
        symbol: StockSymbol::from("STONKS").unwrap(),
    };

    let bytes = request.clone().encode();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'C'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include the firm
    should_be.extend(b"FIRM");
    // Include padded symbol
    should_be.extend(b"STONKS  ");
    // Optional appendage
    should_be.extend(0u16.to_be_bytes());
    assert_eq!(bytes, should_be);


    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().encode();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'C'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // Include the firm
    should_be.extend(b"FIRM");
    // Include padded symbol
    should_be.extend(b"STONKS  ");
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

