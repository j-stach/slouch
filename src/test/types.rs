
use crate::error::BadElementError;

use crate::types::{
    AiqStrategy,
    BboWeightIndicator,
    BrokenReason,
    CancelReason,
    Capacity,
    CrossType,
    CustomerType,
    Display,
    EventCode,
    HandleInst,
    LiquidityFlag,
    OrderState,
    PriceType,
    RestateReason,
    Side,
    TimeInForce,
};

#[test] fn enum_parse_encode() {

    // Test AiqStrategy
    let aiq = AiqStrategy::parse(b'O')
        .expect("Should be valid AiqStrategy");
    assert_eq!(aiq, AiqStrategy::CancelOldestMpid);
    assert_eq!(aiq.encode(), b'O');

    let aiq_error = AiqStrategy::parse(b'X').err()
        .expect("Should be invalid AiqStrategy");
    assert_eq!(
        aiq_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("AiqStrategy")
        )
    );

    // Test BboWeightIndicator
    let bbo = BboWeightIndicator::parse(b'0')
        .expect("Should be valid BboWeightIndicator");
    assert_eq!(bbo, BboWeightIndicator::Micro);
    assert_eq!(bbo.encode(), b'0');

    let bbo_error = BboWeightIndicator::parse(b'X').err()
        .expect("Should be invalid BboWeightIndicator");
    assert_eq!(
        bbo_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("BboWeightIndicator")
        )
    );

    // Test BrokenReason
    let broken = BrokenReason::parse(b'E')
        .expect("Should be valid BrokenReason");
    assert_eq!(broken, BrokenReason::Erroneous);
    assert_eq!(broken.encode(), b'E');

    let broken_error = BrokenReason::parse(b'Z').err()
        .expect("Should be invalid BrokenReason");
    assert_eq!(
        broken_error,
        BadElementError::InvalidEnum(
            String::from('Z'), 
            String::from("BrokenReason")
        )
    );

    // Test CancelReason
    let cancel = CancelReason::parse(b'E')
        .expect("Should be valid CancelReason");
    assert_eq!(cancel, CancelReason::Closed);
    assert_eq!(cancel.encode(), b'E');

    let cancel_error = CancelReason::parse(b'O').err()
        .expect("Should be invalid CancelReason");
    assert_eq!(
        cancel_error,
        BadElementError::InvalidEnum(
            String::from('O'), 
            String::from("CancelReason")
        )
    );

    // Test Capacity
    let cap = Capacity::parse(b'O')
        .expect("Should be valid Capacity");
    assert_eq!(cap, Capacity::Other);
    assert_eq!(cap.encode(), b'O');

    let cap_error = Capacity::parse(b'X').err()
        .expect("Should be invalid Capacity");
    assert_eq!(
        cap_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("Capacity")
        )
    );

    // Test CrossType
    let ct = CrossType::parse(b'O')
        .expect("Should be valid CrossType");
    assert_eq!(ct, CrossType::Opening);
    assert_eq!(ct.encode(), b'O');

    let ct_error = CrossType::parse(b'X').err()
        .expect("Should be invalid CrossType");
    assert_eq!(
        ct_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("CrossType")
        )
    );

    // Test CustomerType
    let ct = CustomerType::parse(b'R')
        .expect("Should be valid CustomerType");
    assert_eq!(ct, CustomerType::Retail);
    assert_eq!(ct.encode(), b'R');

    let ct_error = CustomerType::parse(b'X').err()
        .expect("Should be invalid CustomerType");
    assert_eq!(
        ct_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("CustomerType")
        )
    );

    // Test Display
    let dis = Display::parse(b'N')
        .expect("Should be valid Display");
    assert_eq!(dis, Display::Hidden);
    assert_eq!(dis.encode(), b'N');

    let dis_error = Display::parse(b'X').err()
        .expect("Should be invalid Display");
    assert_eq!(
        dis_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("Display")
        )
    );

    // Test EventCode
    let ec = EventCode::parse(b'E')
        .expect("Should be valid EventCode");
    assert_eq!(ec, EventCode::EndOfDay);
    assert_eq!(ec.encode(), b'E');

    let ec_error = EventCode::parse(b'X').err()
        .expect("Should be invalid EventCode");
    assert_eq!(
        ec_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("EventCode")
        )
    );

    // Test HandleInst
    let hi = HandleInst::parse(b'I')
        .expect("Should be valid HandleInst");
    assert_eq!(hi, HandleInst::ImbalanceOnly);
    assert_eq!(hi.encode(), b'I');

    let hi_error = HandleInst::parse(b'X').err()
        .expect("Should be invalid HandleInst");
    assert_eq!(
        hi_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("HandleInst")
        )
    );

    // Test LiquidityFlag
    let lf = LiquidityFlag::parse(b'A')
        .expect("Should be valid LiquidityFlag");
    assert_eq!(lf, LiquidityFlag::Added);
    assert_eq!(lf.encode(), b'A');

    let lf_error = LiquidityFlag::parse(b'X').err()
        .expect("Should be invalid LiquidityFlag");
    assert_eq!(
        lf_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("LiquidityFlag")
        )
    );

    // Test OrderState
    let os = OrderState::parse(b'L')
        .expect("Should be valid OrderState");
    assert_eq!(os, OrderState::Live);
    assert_eq!(os.encode(), b'L');

    let os_error = OrderState::parse(b'X').err()
        .expect("Should be invalid OrderState");
    assert_eq!(
        os_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("OrderState")
        )
    );

    // Test PriceType
    let pt = PriceType::parse(b'L')
        .expect("Should be valid PriceType");
    assert_eq!(pt, PriceType::Limit);
    assert_eq!(pt.encode(), b'L');

    let pt_error = PriceType::parse(b'X').err()
        .expect("Should be invalid PriceType");
    assert_eq!(
        pt_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("PriceType")
        )
    );

    // Test RestateReason
    let restate = RestateReason::parse(b'R')
        .expect("Should be valid RestateReason");
    assert_eq!(restate, RestateReason::Refresh);
    assert_eq!(restate.encode(), b'R');

    let restate_error = RestateReason::parse(b'X').err()
        .expect("Should be invalid RestateReason");
    assert_eq!(
        restate_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("RestateReason")
        )
    );

    // Test Side
    let side = Side::parse(b'B')
        .expect("Should be valid Side");
    assert_eq!(side, Side::Buy);
    assert_eq!(side.encode(), b'B');

    let side_error = Side::parse(b'X').err()
        .expect("Should be invalid Side");
    assert_eq!(
        side_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("Side")
        )
    );

    // Test TimeInForce
    let tif = TimeInForce::parse(b'0')
        .expect("Should be valid TimeInForce");
    assert_eq!(tif, TimeInForce::Day);
    assert_eq!(tif.encode(), b'0');

    let tif_error = TimeInForce::parse(b'X').err()
        .expect("Should be invalid TimeInForce");
    assert_eq!(
        tif_error,
        BadElementError::InvalidEnum(
            String::from('X'), 
            String::from("TimeInForce")
        )
    );

}

use crate::types::RejectReason;

#[test] fn reject_reason_parse_encode() {

    // Test correct behavior:
    let reject = RejectReason::parse(&0x0026_u16.to_be_bytes())
        .expect("Should be valid RejectReason");
    assert_eq!(reject, RejectReason::RiskFatFinger);
    assert_eq!(reject.encode(), 0x0026_u16.to_be_bytes());

    // Test error behavior:
    let reject_error = RejectReason::parse(&[b'X']).err()
        .expect("Should be invalid RejectReason");
    assert_eq!(
        reject_error,
        BadElementError::WrongSize(String::from("u16"), 2, 1)
    );

    let reject_error = RejectReason::parse(&[b'X'; 3]).err()
        .expect("Should be invalid RejectReason");
    assert_eq!(
        reject_error,
        BadElementError::WrongSize(String::from("u16"), 2, 3)
    );

    let reject_error = RejectReason::parse(&0x0034_u16.to_be_bytes()).err()
        .expect("Should be invalid RejectReason");
    assert_eq!(
        reject_error,
        BadElementError::InvalidEnum(
            format!("{:#x}", 0x0034),
            String::from("RejectReason")
        )
    );

}

use crate::types::{
    BrokerId,
    RouteId,
};

#[test] fn route_broker_new() {

    // Test correct BrokerId behavior:
    let bid = BrokerId::new("Good")
        .expect("Should be valid BrokerId");
    assert_eq!(bid.to_string(), String::from("Good"));

    let bid = BrokerId::new("Ok")
        .expect("Should be valid BrokerId");
    assert_eq!(bid.to_string(), String::from("Ok"));

    // Test BrokerId error behavior:
    let bid_error = BrokerId::new("TooLong").err()
        .expect("Should be invalid BrokerId");
    assert_eq!(
        bid_error,
        BadElementError::WrongSize(String::from("BrokerId"), 4, 7)
    );

    let bid_error = BrokerId::new("B4d").err()
        .expect("Should be invalid BrokerId");
    assert_eq!(
        bid_error,
        BadElementError::NotAlpha(String::from("BrokerId"))
    );

    // Test correct RouteId behavior:
    let rid = RouteId::new("Good")
        .expect("Should be valid RouteId");
    assert_eq!(rid.to_string(), String::from("Good"));

    let rid = RouteId::new("Ok")
        .expect("Should be valid RouteId");
    assert_eq!(rid.to_string(), String::from("Ok"));

    // Test RouteId error behavior:
    let rid_error = RouteId::new("TooLong").err()
        .expect("Should be invalid RouteId");
    assert_eq!(
        rid_error,
        BadElementError::WrongSize(String::from("RouteId"), 4, 7)
    );

    let rid_error = RouteId::new("B4d").err()
        .expect("Should be invalid RouteId");
    assert_eq!(
        rid_error,
        BadElementError::NotAlpha(String::from("RouteId"))
    );

}

#[test] fn route_broker_parse_encode() {

    // Test BrokerId:
    let bid_bytes = b"Good";
    let bid = BrokerId::parse(bid_bytes)
        .expect("Should be valid BrokerId");
    assert_eq!(bid.to_string(), String::from("Good"));
    assert_eq!(bid.encode(), *bid_bytes);

    // Test RouteId:
    let rid_bytes = b"Good";
    let rid = RouteId::parse(rid_bytes)
        .expect("Should be valid RouteId");
    assert_eq!(rid.to_string(), String::from("Good"));
    assert_eq!(rid.encode(), *rid_bytes);

}

use crate::types::{
    FirmId,
    OrderToken,
    StockSymbol
};

#[test] fn firm_new_parse_encode() {
    
    // Test correct behavior:
    let fid = FirmId::new("FIRM")
        .expect("Should be valid FormId");
    assert_eq!(fid.to_string(), String::from("FIRM"));

    let fid_bytes = b"FIRM";
    let fid = FirmId::parse(fid_bytes)
        .expect("Should be valid FirmId");
    assert_eq!(fid.encode(), *fid_bytes);

    // Test new error behavior:
    let error = FirmId::new("TOOLONG").err()
        .expect("Should be invalid FormId");
    assert_eq!(
        error,
        BadElementError::WrongSize(String::from("FirmId"), 4, 7)
    );


    let error = FirmId::new("BAD").err()
        .expect("Should be invalid FormId");
    assert_eq!(
        error,
        BadElementError::WrongSize(String::from("FirmId"), 4, 3)
    );


    let error = FirmId::new("Firm").err()
        .expect("Should be invalid FormId");
    assert_eq!(
        error,
        BadElementError::NotUppercaseAlpha(String::from("FirmId"))
    );

    let error = FirmId::new("F1RM").err()
        .expect("Should be invalid FormId");
    assert_eq!(
        error,
        BadElementError::NotUppercaseAlpha(String::from("FirmId"))
    );

    // Test parse error behavior:
    let bad_bytes = b"Firm";
    let error = FirmId::parse(bad_bytes).err()
        .expect("Should be invalid FirmId");
    assert_eq!(
        error,
        BadElementError::NotUppercaseAlpha(String::from("FirmId"))
    );
}

#[test] fn order_token_new_parse_encode() {

    // Test correct behavior:
    let ot = OrderToken::new("Test")
        .expect("Should be valid OrderToken");
    assert_eq!(ot.to_string(), String::from("Test"));

    let ot_bytes = b"Test";
    let ot = OrderToken::parse(ot_bytes)
        .expect("Should be valid OrderToken");
    assert_eq!(ot.encode(), *b"Test          ");

    let ot = OrderToken::new("Test 2")
        .expect("Should be valid OrderToken");
    assert_eq!(ot.to_string(), String::from("Test 2"));

    let ot_bytes = b"Test 2";
    let ot = OrderToken::parse(ot_bytes)
        .expect("Should be valid OrderToken");
    assert_eq!(ot.encode(), *b"Test 2        ");

    // Test new error behavior:
    let error = OrderToken::new("abcdefghijklmno").err()
        .expect("Should be invalid OrderToken");
    assert_eq!(
        error,
        BadElementError::WrongSize(String::from("OrderToken"), 14, 15)
    );
}

#[test] fn symbol_new_parse_encode() {

    // Test correct behavior:
    let sym = StockSymbol::new("Test")
        .expect("Should be valid StockSymbol");
    assert_eq!(sym.to_string(), String::from("Test"));

    let sym_bytes = b"Test";
    let sym = StockSymbol::parse(sym_bytes)
        .expect("Should be valid StockSymbol");
    assert_eq!(sym.encode(), *b"Test    ");

    // Test new error behavior:
    let error = StockSymbol::new("waytoolong").err()
        .expect("Should be invalid StockSymbol");
    assert_eq!(
        error,
        BadElementError::WrongSize(String::from("StockSymbol"), 8, 10)
    );

    let error = StockSymbol::new("T3st").err()
        .expect("Should be invalid StockSymbol");
    assert_eq!(
        error,
        BadElementError::NotAlpha(String::from("StockSymbol"))
    );

    // Test parse error behavior:
    let sym_bytes = b"T3st";
    let error = StockSymbol::parse(sym_bytes).err()
        .expect("Should be valid StockSymbol");
    assert_eq!(
        error,
        BadElementError::NotAlpha(String::from("StockSymbol"))
    );
}

use crate::types::{
    Price,
    SignedPrice,
    ElapsedTime,
};

#[test] fn price_new_parse_encode() {

    // Test correct Price behavior:
    let price = Price::new(0u32, 0u16)
        .expect("Should be valid Price");
    let parsed = Price::parse(&[0; 8])
        .expect("Should be valid Price");
    assert_eq!(price, parsed);

    let price = Price::new(99u32, 9900u16)
        .expect("Should be valid Price");
    let parsed = Price::parse(&999900u64.to_be_bytes())
        .expect("Should be valid Price");
    assert_eq!(price, parsed);

    // TODO: Test market & market cross

    // Test Price error behavior:
    let error = Price::new(200_000_u32, 0u16).err()
        .expect("Should be invalid Price");
    assert_eq!(
        error,
        BadElementError::InvalidValue(String::from("Price"))
    );

    let error = Price::new(0u32, 59999u16).err()
        .expect("Should be invalid Price");
    assert_eq!(
        error,
        BadElementError::InvalidValue(String::from("Price"))
    );

    // Test correct SignedPrice behavior:
    let price = SignedPrice::new(0u32, 0u16, false)
        .expect("Should be valid Price");
    let parsed = SignedPrice::parse(&[0; 4])
        .expect("Should be valid Price");
    assert_eq!(price, parsed);

    let price = SignedPrice::new(99u32, 9900u16, true)
        .expect("Should be valid Price");
    let parsed = SignedPrice::parse(&(-999900i32).to_be_bytes())
        .expect("Should be valid Price");
    assert_eq!(price, parsed);

    // Test SignedPrice error behavior:
    let error = SignedPrice::new(0u32, 59999u16, true).err()
        .expect("Should be invalid Price");
    assert_eq!(
        error,
        BadElementError::InvalidValue(String::from("SignedPrice"))
    );

    // TODO: TBD, SignedPrice may have additional undisclosed bound errors
}

#[test] fn time_new_parse_encode() {
    // Test correct behavior:
    let time = ElapsedTime::new(0u32)
        .expect("Should be valid ElapsedTime");
    assert_eq!(time.secs(), 0u32);

    let time = ElapsedTime::new(86_399_u32)
        .expect("Should be valid ElapsedTime");
    assert_eq!(time.secs(), 86_399_u32);

    let time_bytes = u32::MIN.to_be_bytes();
    let time = ElapsedTime::parse(&time_bytes)
        .expect("Should be valid ElapsedTime");
    assert_eq!(time.secs(), 0u32);

    // Test error behavior:
    let error = ElapsedTime::new(86_400_u32).err()
        .expect("Should be invalid ElapsedTime");
    assert_eq!(
        error,
        BadElementError::InvalidValue(String::from("ElapsedTime"))
    );

    let time_bytes = b"zzzzz";
    let error = ElapsedTime::parse(time_bytes).err()
        .expect("Should be invalid ElapsedTime");
    assert_eq!(
        error,
        BadElementError::WrongSize(String::from("u32"), 4, 5)
    );

    let time_bytes = u32::MAX.to_be_bytes();
    let error = ElapsedTime::parse(&time_bytes).err()
        .expect("Should be invalid ElapsedTime");
    assert_eq!(
        error,
        BadElementError::InvalidValue(String::from("ElapsedTime"))
    );

}

use crate::types::UserRefNum;

#[test] fn user_ref_num() {
    
    // UserRefNum must begin at 1.
    let mut urn = UserRefNum::new();
    assert_eq!(urn.val(), 1u32);

    urn.increment();
    assert_eq!(urn.val(), 2u32);

    // Test correct behavior:
    let urn = UserRefNum::parse(&[b'z'; 4])
        .expect("Should be valid u32");

    let bytes = urn.encode();
    assert_eq!(bytes, [b'z'; 4]);

    // Test error behavior:
    let error = UserRefNum::parse(&[b'z'; 5]).err()
        .expect("Should be invalid u32");
    assert_eq!(
        error, 
        BadElementError::WrongSize(String::from("u32"), 4, 5)
    );

    let error = UserRefNum::parse(&[b'z'; 3]).err()
        .expect("Should be invalid u32");
    assert_eq!(
        error, 
        BadElementError::WrongSize(String::from("u32"), 4, 3)
    );

}

