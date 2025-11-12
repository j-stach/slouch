
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
        symbol: StockSymbol::from("STONKS").unwrap(),
        price: Price::new(35001).unwrap(),
        time_in_force: TimeInForce::Day,
        display: Display::Visible,
        capacity: Capacity::Agency,
        intermarket_sweep: false,
        cross_type: CrossType::Opening,
        order_token: OrderToken::from("To The Moon").unwrap()
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

// Tests `Firm` and adding multiple options to a request.
#[test] fn firm_multiple() { 

    let root = test_order();

    let mut enter = root.0;
    enter.add_option(
        TagValue::Firm(Mpid::from("FIRM").unwrap()
    )).unwrap();
    enter.add_option(TagValue::UserRefIndex(1u8)).unwrap();
    let bytes = enter.encode();

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

// Tests `MaxFloor` and replacing an option in the appendage.
#[test] fn max_floor_replace() { 

    let root = test_order();

    let mut enter = root.0;
    // (This gets overwritten to 1u32 below)
    enter.add_option(TagValue::MaxFloor(0u32)).unwrap();
    // Overwrite an option with a new value to ensure only it is encoded
    // and that it is encoded in the same order as originally added.
    enter.add_option(TagValue::MaxFloor(1u32)).unwrap();
    let bytes = enter.encode();

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

#[test] fn min_qty() { 

    let min_qty = TagValue::MinQty(0u32);
    let bytes = min_qty.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(3u8);
    // MinQty bytes
    should_be.extend(0u32.to_be_bytes());

    assert_eq!(bytes, should_be);
}
    
#[test] fn customer_type() { 

    let ct = TagValue::Retail(Some(true));
    let bytes = ct.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(4u8);
    // CustomerType flag
    should_be.push(b'R');

    assert_eq!(bytes, should_be);
}

#[test] fn price_type() { 

    let pt = TagValue::PriceType(PriceType::Limit);
    let bytes = pt.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(6u8);
    // PriceTypes flag
    should_be.push(b'L');

    assert_eq!(bytes, should_be);
}

#[test] fn peg_offset() { 

    let po = TagValue::PegOffset(SignedPrice::new(-200i32).unwrap());
    let bytes = po.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(7u8);
    // PegOffest price bytes
    should_be.extend((-200i32).to_be_bytes());

    assert_eq!(bytes, should_be);
}

// Tests all discretion price related options together.
#[test] fn discretion_price_etc() { 

    let root = test_order();

    let mut enter = root.0;
    // - DiscretionPrice
    enter.add_option(
        TagValue::DiscretionPrice(Price::new(200u64).unwrap())
    ).unwrap();
    // - DiscretionPriceType
    enter.add_option(
        TagValue::DiscretionPriceType(PriceType::Limit)
    ).unwrap();
    // - DiscretionPegOffset
    enter.add_option(
        TagValue::DiscretionPegOffset(
            SignedPrice::new(-200i32)
            .unwrap()
        )
    ).unwrap();
    let bytes = enter.encode();

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

    let po = TagValue::PostOnly(false);
    let bytes = po.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(12u8);
    // PostOnly flag
    should_be.push(b'N');

    assert_eq!(bytes, should_be);
}

#[test] fn random_reserves() { 

    let rr = TagValue::RandomReserves(0u32);
    let bytes = rr.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(13u8);
    // RansomReserves quantity bytes
    should_be.extend(0u32.to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn expire_time() { 

    let et = TagValue::ExpireTime(ElapsedTime::new(420u32).unwrap());
    let bytes = et.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(15u8);
    // ExpireTime bytes
    should_be.extend(420u32.to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn trade_now() { 

    let tn = TagValue::TradeNow(Some(true));
    let bytes = tn.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(16u8);
    // TradeNow flag
    should_be.push(b'Y');

    assert_eq!(bytes, should_be);
}

#[test] fn handle_inst() { 

    let hi = TagValue::HandleInst(HandleInst::ImbalanceOnly);
    let bytes = hi.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(17u8);
    // HandleInst flag
    should_be.push(b'I');

    assert_eq!(bytes, should_be);
}

#[test] fn group_id() { 

    let gid = TagValue::GroupId(1u16);
    let bytes = gid.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(3u8);
    // Include option tag
    should_be.push(24u8);
    // GroupId bytes
    should_be.extend(1u16.to_be_bytes());

    assert_eq!(bytes, should_be);
}

// Tests locate options together
#[test] fn locate_shares_broker() { 

    let root = test_order();

    let mut enter = root.0;
    // - SharesLocated
    enter.add_option(TagValue::SharesLocated(true)).unwrap();
    // - LocateBroker
    enter.add_option(
        TagValue::LocateBroker(Mpid::from("XXXX").unwrap())
    ).unwrap();
    let bytes = enter.encode();

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

#[test] fn secondary_ord_ref_num() { 

    let sorn = TagValue::SecondaryOrdRefNum(1u64);
    let bytes = sorn.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(9u8);
    // Include option tag
    should_be.push(1u8);
    // OrdRefNum bytes
    should_be.extend(1u64.to_be_bytes());

    assert_eq!(bytes, should_be);
}

#[test] fn bbo_weight() { 

    let bbo = TagValue::BboWeight(BboWeight::Large);
    let bytes = bbo.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(2u8);
    // Include option tag
    should_be.push(18u8);
    // Indicator flag
    should_be.push(b'3');

    assert_eq!(bytes, should_be);
}

#[test] fn display_quantity_price() { 

    let q = TagValue::DisplayQuantity(1u32);
    let bytes = q.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(5u8);
    // Include option tag
    should_be.push(22u8);
    // OrdRefNum bytes
    should_be.extend(1u32.to_be_bytes());

    assert_eq!(bytes, should_be);

    let p = TagValue::DisplayPrice(Price::new(1u64).unwrap());
    let bytes = p.encode();

    let mut should_be = vec![];

    // Include the tag value length marker
    should_be.push(9u8);
    // Include option tag
    should_be.push(23u8);
    // OrdRefNum bytes
    should_be.extend(1u64.to_be_bytes());

    assert_eq!(bytes, should_be);
}


