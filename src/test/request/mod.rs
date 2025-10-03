
#[cfg(test)] mod cancel;
#[cfg(test)] mod query;

use crate::{ msg::OuchRequest, account_query };

#[test] fn request_new_encode() {

    // Macros are tested in the doc comments 
    let request = account_query!();

    let bytes = request.clone().to_bytes();

    let mut should_be: Vec<u8> = vec![b'Q'];
    should_be.extend(0u16.to_be_bytes());

    assert_eq!(bytes, &should_be[0..3]);
}

// TODO: Test encodings for other messages

