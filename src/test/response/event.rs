
use crate::msg::OuchResponse;
use crate::types::EventCode;

use nsdq_util::types::time::Timelike;

#[test] fn parse_system_event() {

    let mut data: Vec<u8> = vec![b'S'];
    // u64 for timestamp
    data.extend(1u64.to_be_bytes());
    // S for event code
    data.push(b'S');

    let (_data, response) = OuchResponse::parse(&data[0..])
        .expect("Should be valid data");

    let event = match response {
        OuchResponse::SystemEvent(event) => event,
        _ => panic!("Damn, son, where'd you find this?")
    };

    assert_eq!(event.timestamp().nanosecond(), 1u32);
    assert_eq!(event.event_code(), EventCode::StartOfDay);
}

