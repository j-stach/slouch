
use crate::msg::{ OuchResponse, TagValue };
use crate::types::*;

use chrono::Timelike;


#[test] fn parse_canceled() {

    let mut data: Vec<u8> = vec![b'C'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // u32 for quantity
    data.extend(0u32.to_be_bytes());
    // Include cancel reason
    data.push(b'H');
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

    let cancel = match response {
        OuchResponse::OrderCanceled(cancel) => cancel,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(cancel.timestamp().nanosecond(), 1u32);
    assert_eq!(cancel.user_ref_num().val(), 1u32);
    assert_eq!(cancel.reason(), CancelReason::Halted);
    assert_eq!(cancel.quantity(), 0u32);

    let options = cancel.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

#[test] fn parse_aiq_canceled() {

    let mut data: Vec<u8> = vec![b'D'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // u32 for UserRefNum
    data.extend(1u32.to_be_bytes());
    // u32 for decrement shares
    data.extend(0u32.to_be_bytes());
    // Include cancel reason
    data.push(b'Q');
    // u32 for quantity prevented
    data.extend(0u32.to_be_bytes());
    // execution price
    data.extend(35001u64.to_be_bytes());
    // liquidity flag
    data.push(b'A');
    // aiq strategy
    data.push(b'O');
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

    let cancel = match response {
        OuchResponse::AiqCanceled(cancel) => cancel,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(cancel.timestamp().nanosecond(), 1u32);
    assert_eq!(cancel.user_ref_num().val(), 1u32);
    assert_eq!(cancel.reason(), CancelReason::SelfMatchPrevention);
    assert_eq!(cancel.quantity_prevented(), 0u32);
    assert_eq!(cancel.execution_price().dollars(), 3u32);
    assert_eq!(cancel.execution_price().cents(), 5001u16);
    assert_eq!(cancel.decrement_shares(), 0u32);
    assert_eq!(cancel.liquidity_flag(), LiquidityFlag::Added);
    assert_eq!(cancel.aiq_strategy(), AiqStrategy::CancelOldestMpid);

    let options = cancel.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

#[test] fn parse_pending() {

    let mut data: Vec<u8> = vec![b'P'];
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

    let response = OuchResponse::try_from(&data[0..])
        .expect("Should be valid data");

    let cancel = match response {
        OuchResponse::CancelPending(cancel) => cancel,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(cancel.timestamp().nanosecond(), 1u32);
    assert_eq!(cancel.user_ref_num().val(), 1u32);

    let options = cancel.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

#[test] fn parse_rejected() {

    let mut data: Vec<u8> = vec![b'I'];
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

    let response = OuchResponse::try_from(&data[0..])
        .expect("Should be valid data");

    let cancel = match response {
        OuchResponse::CancelRejected(cancel) => cancel,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(cancel.timestamp().nanosecond(), 1u32);
    assert_eq!(cancel.user_ref_num().val(), 1u32);

    let options = cancel.options();
    match options[0] {
        TagValue::UserRefIndex(val) => {
            assert_eq!(val, 0u8);
        },
        _ => panic!("Damn, son, where'd you find this?")
    };
}

