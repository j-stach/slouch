
use crate::{ 
    replace, 
    msg::{ TagValue, OuchRequest },
};

use crate::types::*;

#[test] fn new_replace() {

    // Macros are tested in the doc comments 
    let mut request = replace!{
        user_ref_num: UserRefNum::new(),
        quantity: 0u32,
        price: Price::new(3, 5001).unwrap(),
        time_in_force: TimeInForce::Day,
        display: Display::Visible,
        intermarket_sweep_eligibility: false,
        order_token: OrderToken::new("OrderToken").unwrap()
    };
    
    let eo = match request {
        OuchRequest::ReplaceOrder(ref eo) => eo,
        _ => panic!{"Damn son, where'd you find that"}
    };
    assert_eq!(eo.user_ref_num(), UserRefNum::new());
    assert_eq!(eo.quantity(), 0u32);
    assert_eq!(eo.price(), Price::new(3, 5001).unwrap());
    assert_eq!(eo.time_in_force(), TimeInForce::Day);
    assert_eq!(eo.display(), Display::Visible);
    assert_eq!(eo.intermarket_sweep_eligibility(), false);
    assert_eq!(eo.order_token(), OrderToken::new("OrderToken").unwrap());
    assert!(request.options().is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    assert!(!request.options().is_empty());
}

#[test]
#[should_panic]
fn bad_quantity() {
    let _ = replace!{
        user_ref_num: UserRefNum::new(),
        quantity: 1_000_000u32,
        price: Price::new(3, 5001).unwrap(),
        time_in_force: TimeInForce::Day,
        display: Display::Visible,
        intermarket_sweep_eligibility: false,
        order_token: OrderToken::new("To The Moon").unwrap()
    };
}

#[test] fn encode_replace() {

    let mut request = replace!{
        user_ref_num: UserRefNum::new(),
        quantity: 0u32,
        price: Price::new(3, 5001).unwrap(),
        time_in_force: TimeInForce::Day,
        display: Display::Visible,
        intermarket_sweep_eligibility: false,
        order_token: OrderToken::new("To The Moon").unwrap()
    };
    
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'U'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // u32 for quantity
    should_be.extend(0u32.to_be_bytes());
    // Price
    should_be.extend(35001u64.to_be_bytes());
    // Time in Force
    should_be.push(b'0');
    // Display
    should_be.push(b'Y');
    // ISE
    should_be.push(b'N');
    // CIOrdId
    should_be.extend(b"To The Moon   ");
    // Optional appendage
    should_be.extend(0u16.to_be_bytes());
    assert_eq!(bytes, should_be);


    request.add_option(TagValue::UserRefIndex(1u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().to_bytes();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'U'];
    // u32 for UserRefNum
    should_be.extend(1u32.to_be_bytes());
    // u32 for quantity
    should_be.extend(0u32.to_be_bytes());
    // Price
    should_be.extend(35001u64.to_be_bytes());
    // Time in Force
    should_be.push(b'0');
    // Display
    should_be.push(b'Y');
    // ISE
    should_be.push(b'N');
    // CIOrdId
    should_be.extend(b"To The Moon   ");
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


