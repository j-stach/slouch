
use crate::{ account_query, msg::TagValue };

#[test] fn account_query() {

    let mut request = account_query!();
    let options = request.options();
    assert!(options.is_empty());

    request.add_option(TagValue::UserRefIndex(0u8))
        .expect("Should probably be a good optional value");
    let options = request.options();
    assert!(!options.is_empty());
}

// TODO: Encode?

