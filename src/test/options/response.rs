
use crate::{ 
    msg::TagValue,
    types::*,
};


#[test] fn discretion_price() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![9u8];
    // OrdRefNum bytes
    bytes.extend(200u64.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::DiscretionPrice(val) => assert_eq!(
            val, 
            Price::new(0, 200).unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn discretion_price_type() {

    // Include the option tag, ignore tag value length marker 
    let mut bytes = vec![10u8];
    // PriceTypes flag
    bytes.push(b'L');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::DiscretionPriceType(val) => assert_eq!(
            val, PriceType::Limit
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn discretion_peg_offset() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![11u8];
    // PegOffest price bytes
    bytes.extend((-200i32).to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::DiscretionPegOffset(val) => assert_eq!(
            val, 
            SignedPrice::new(0, 200, true).unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}


#[test] fn firm() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![2u8];
    // Include the firm chars
    bytes.extend(b"XXXX");

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::Firm(val) => assert_eq!(
            val, 
            FirmId::new("XXXX").unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn max_floor() {

    // Include the option tag, ignore tag value length marker 
    let mut bytes = vec![5u8];
    // MinQty bytes
    bytes.extend(0u32.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::MaxFloor(val) => assert_eq!(val, 0u32),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn min_qty() {

    // Include the option tag, ignore tag value length marker 
    let mut bytes = vec![3u8];
    // MinQty bytes
    bytes.extend(0u32.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::MinQty(val) => assert_eq!(val, 0u32),
        _ => panic!("Where'd you find this?"),
    }

}
    
#[test] fn customer_type() {

    // Include the option tag, ignore tag value length marker 
    let mut bytes = vec![4u8];
    // CustomerType flag
    bytes.push(b'R');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::CustomerType(val) => assert_eq!(val, CustomerType::Retail),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn price_type() {

    // Include the option tag, ignore tag value length marker 
    let mut bytes = vec![6u8];
    // PriceTypes flag
    bytes.push(b'L');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::PriceType(val) => assert_eq!(val, PriceType::Limit),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn peg_offset() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![7u8];
    // PegOffest price bytes
    bytes.extend((-200i32).to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::PegOffset(val) => assert_eq!(
            val, 
            SignedPrice::new(0, 200, true).unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}
    

#[test] fn post_only() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![12u8];
    // PostOnly flag
    bytes.push(b'N');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::PostOnly(val) => assert_eq!(val, false),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn random_reserves() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![13u8];
    // RansomReserves quantity bytes
    bytes.extend(0u32.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::RandomReserves(val) => assert_eq!(val, 0u32),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn expire_time() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![15u8];
    // ExpireTime bytes
    bytes.extend(420u32.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::ExpireTime(val) => assert_eq!(
            val, 
            ElapsedTime::new(420u32).unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn trade_now() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![16u8];
    // TradeNow flag
    bytes.push(b'Y');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::TradeNow(val) => assert_eq!(val, true),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn handle_inst() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![17u8];
    // HandleInst flag
    bytes.push(b'I');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::HandleInst(val) => assert_eq!(
            val, 
            HandleInst::ImbalanceOnly
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn group_id() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![24u8];
    // GroupId bytes
    bytes.extend(1u16.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::GroupId(val) => assert_eq!(val, 1u16),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn locate_broker() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![26u8];
    // Include the broker chars
    bytes.extend(b"XXXX");

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::LocateBroker(val) => assert_eq!(
            val, 
            BrokerId::new("XXXX").unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn locate_shares() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![25u8];
    // SharedLocated flag
    bytes.push(b'Y');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::SharesLocated(val) => assert_eq!(val, true),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn secondary_ord_ref_num() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![1u8];
    // OrdRefNum bytes
    bytes.extend(1u64.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::SecondaryOrdRefNum(val) => assert_eq!(val, 1u64),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn bbo_weight_indicator() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![18u8];
    // Indicator flag
    bytes.push(b'3');

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::BboWeightIndicator(val) => assert_eq!(
            val, 
            BboWeightIndicator::Large,
        ),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn display_quantity() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![22u8];
    // OrdRefNum bytes
    bytes.extend(1u32.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::DisplayQuantity(val) => assert_eq!(val, 1u32),
        _ => panic!("Where'd you find this?"),
    }

}

#[test] fn display_price() {

    // Include the option tag, ignore length marker
    let mut bytes = vec![23u8];
    // OrdRefNum bytes
    bytes.extend(200u64.to_be_bytes());

    let tv = TagValue::parse(&bytes).unwrap();
    match tv {
        TagValue::DisplayPrice(val) => assert_eq!(
            val, 
            Price::new(0, 200).unwrap()
        ),
        _ => panic!("Where'd you find this?"),
    }

}

