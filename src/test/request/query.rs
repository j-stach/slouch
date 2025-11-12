
use crate::{ account_query, msg::TagValue };

#[test] fn new() {

    // Macros are tested in the doc comments 
    let mut request = account_query!();
    let options = request.options();
    assert!(options.is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    let options = request.options();
    assert!(!options.is_empty());
}

#[test] fn encode() {

    let mut request = account_query!();

    let bytes = request.clone().encode();

    let mut should_be: Vec<u8> = vec![b'Q'];
    // Opts length (empty)
    should_be.extend(0u16.to_be_bytes());
    assert_eq!(bytes, should_be);

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should be a good optional value");
    let bytes = request.clone().encode();

    // Include the request type tag
    let mut should_be: Vec<u8> = vec![b'Q'];
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

