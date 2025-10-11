
use crate::{ 
    enter,
    msg::TagValue,
    types::*,
};

// Helper function with default message and encoding
// (minus optional appendage)
fn test_order() -> (crate::msg::OuchRequest, Vec<u8>) {

    let order = enter!{
        user_ref_num: UserRefNum::new(),
        side: Side::Buy,
        quantity: 69u32,
        symbol: StockSymbol::new("STONKS").unwrap(),
        price: Price::new(3, 5001).unwrap(),
        time_in_force: TimeInForce::Day,
        display: Display::Visible,
        capacity: Capacity::Agency,
        intermarket_sweep_eligibility: false,
        cross_type: CrossType::Opening,
        order_token: OrderToken::new("To The Moon").unwrap()
    };

    // Request message reconstruction byte-wise:
    // Include the request type tag
    let mut bytes: Vec<u8> = vec![b'O'];
    // u32 for UserRefNum
    bytes.extend(1u32.to_be_bytes());
    // Include side flag
    bytes.push(b'B');
    // u32 for quantity
    bytes.extend(69u32.to_be_bytes());
    // Symbol
    bytes.extend(b"STONKS  ");
    // Price
    bytes.extend(35001u64.to_be_bytes());
    // Time in Force
    bytes.push(b'0');
    // Display
    bytes.push(b'Y');
    // Capacity
    bytes.push(b'A');
    // ISE
    bytes.push(b'N');
    // Cross Type
    bytes.push(b'O');
    // CIOrdId
    bytes.extend(b"To The Moon   ");

    (order, bytes)
}


#[test] fn firm_multiple() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(
        TagValue::Firm(FirmId::new("FIRM").unwrap()
    )).unwrap();
    enter.add_option(TagValue::UserRefIndex(1u8)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(9u16.to_be_bytes());

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(2u8);
    // Include the firm chars
    should_be.extend(b"FIRM");

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(28u8);
    // Include the UserRefIndex byte 
    should_be.push(1u8);

    assert_eq!(bytes, should_be);
}

#[test] fn min_qty() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(TagValue::MinQty(0u32)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(6u16.to_be_bytes());

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(3u8);
    // MinQty bytes
    should_be.extend(0u32.to_be_bytes());

    assert_eq!(bytes, should_be);
}
    
#[test] fn customer_type() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(
        TagValue::CustomerType(CustomerType::Retail)
    ).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(4u8);
    // CustomerType flag
    should_be.push(b'R');

    assert_eq!(bytes, should_be);
}

#[test] fn max_floor_replace() { 

    let mut root = test_order();

    let mut enter = root.0;
    // (This gets overwritten to 1u32 below)
    enter.add_option(TagValue::MaxFloor(0u32)).unwrap();
    // Overwrite an option with a new value to ensure only it is encoded
    // and that it is encoded in the same order as originally added.
    enter.add_option(TagValue::MaxFloor(1u32)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(6u16.to_be_bytes());

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(5u8);
    // MaxFloor bytes
    should_be.extend(1u32.to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn price_type() { 

    let mut root = test_order();

    let mut enter = root.0;
    // - PriceType
    enter.add_option(TagValue::PriceType(PriceType::Limit)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(6u8);
    // PriceTypes flag
    should_be.push(b'L');

    assert_eq!(bytes, should_be);
}

#[test] fn peg_offset() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(
        TagValue::PegOffset(SignedPrice::new(0, 0200, true).unwrap())
    ).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(6u16.to_be_bytes());

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(7u8);
    // PegOffest price bytes
    should_be.extend((-200i32).to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn discretion_price_etc() { 

    let mut root = test_order();

    let mut enter = root.0;
    // - DiscretionPrice
    enter.add_option(
        TagValue::DiscretionPrice(Price::new(0, 0200).unwrap())
    ).unwrap();
    // - DiscretionPriceType
    enter.add_option(
        TagValue::DiscretionPriceType(PriceType::Limit)
    ).unwrap();
    // - DiscretionPegOffset
    enter.add_option(
        TagValue::DiscretionPegOffset(
            SignedPrice::new(0, 0200, true)
            .unwrap()
        )
    ).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(19u16.to_be_bytes());

    // - DiscretionPrice
    // Include the tag value length marker
    should_be.push(9u8);
    // Include option tag
    should_be.push(9u8);
    // DiscretionPrice price bytes
    should_be.extend(200u64.to_be_bytes());

    // - DiscretionPriceType
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(10u8);
    // DiscretionPriceType flag
    should_be.push(b'L');

    // - DiscretionPegOffset
    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(11u8);
    // DiscretionPegOffest price bytes
    should_be.extend((-200i32).to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn post_only() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(TagValue::PostOnly(false)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());

    // - PostOnly
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(12u8);
    // PostOnly flag
    should_be.push(b'N');

    assert_eq!(bytes, should_be);
}

#[test] fn random_reserves() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(TagValue::RandomReserves(0u32)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(6u16.to_be_bytes());

    // - RandomReserves
    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(13u8);
    // RansomReserves quantity bytes
    should_be.extend(0u32.to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn expire_time() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(
        TagValue::ExpireTime(ElapsedTime::new(420u32).unwrap())
    ).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(6u16.to_be_bytes());

    // - ExpireTime
    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(15u8);
    // ExpireTime bytes
    should_be.extend(420u32.to_be_bytes());
}

#[test] fn trade_now() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(TagValue::TradeNow(true)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());

    // - TradeNow
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(16u8);
    // TradeNow flag
    should_be.push(b'Y');

    assert_eq!(bytes, should_be);
}

#[test] fn handle_inst() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(
        TagValue::HandleInst(HandleInst::ImbalanceOnly)
    ).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(3u16.to_be_bytes());

    // - HandleInst
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(17u8);
    // HandleInst flag
    should_be.push(b'I');

    assert_eq!(bytes, should_be);
}

#[test] fn group_id() { 

    let mut root = test_order();

    let mut enter = root.0;
    enter.add_option(TagValue::GroupId(1u16)).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(4u16.to_be_bytes());

    // - GroupId
    // Include the tag value length marker
    should_be.push(3u8);
    // Include option tag
    should_be.push(24u8);
    // GroupId bytes
    should_be.extend(1u16.to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn locate_shares_broker() { 

    let mut root = test_order();

    let mut enter = root.0;
    // - SharesLocated
    enter.add_option(TagValue::SharesLocated(true)).unwrap();
    // - LocateBroker
    enter.add_option(
        TagValue::LocateBroker(BrokerId::new("XXXX").unwrap())
    ).unwrap();
    let bytes = enter.to_bytes();

    let mut should_be = root.1;
    // Include the appendage length marker
    should_be.extend(9u16.to_be_bytes());

    // - SharesLocated
    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(25u8);
    // SharedLocated flag
    should_be.push(b'Y');

    // - LocateBroker
    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(26u8);
    // Include the broker chars
    should_be.extend(b"XXXX");

    assert_eq!(bytes, should_be);
}

// TODO: Figure out which messages to use for the options not covered

// TODO: Create an OrderAccepted with all possible options to test parse
// Ensure order does not matter



