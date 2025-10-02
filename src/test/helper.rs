
use crate::error::BadElementError;

use crate::helper::{
    check_alpha_compliance,
    check_alpha_uppercase,
};

#[test] fn check_alpha_compliance_test() {

    // These should be alpha compliant.
    let s1 = String::from("AAAA");
    let s2 = String::from("AaAa");
    let s3 = String::from("aaaa");
    let s4 = String::from("aaa");

    // These should not be alpha compliant.
    let too_long = String::from("abcde");
    let has_digits = String::from("1234");
    let has_ws1 = String::from("a aa");
    let has_ws2 = String::from("aaa ");
    let not_ascii = String::from("aaaă");

    // Test correct behavior:
    let _ = check_alpha_compliance(&s1, 4, "s1").expect("s1 is compliant");
    let _ = check_alpha_compliance(&s2, 4, "s2").expect("s2 is compliant");
    let _ = check_alpha_compliance(&s3, 4, "s3").expect("s3 is compliant");
    let _ = check_alpha_compliance(&s4, 4, "s4").expect("s4 is compliant");

    // Test error behavior:
    let error = check_alpha_compliance(&too_long, 4, "too_long").err()
        .expect("too_long is error");
    assert_eq!(
        error, 
        BadElementError::WrongSize(String::from("too_long"), 4, 5)
    );

    let error = check_alpha_compliance(&has_digits, 4, "has_digits").err()
        .expect("has_digits is error");
    assert_eq!(
        error, 
        BadElementError::NotAlpha(String::from("has_digits"))
    );

    let _ = check_alpha_compliance(&has_ws1, 4, "has_ws1").err()
        .expect("has_ws1 is error");
    let _ = check_alpha_compliance(&has_ws2, 4, "has_ws2").err()
        .expect("has_ws2 is error");
    let _ = check_alpha_compliance(&not_ascii, 4, "not_ascii").err()
        .expect("not_ascii is error");
}

#[test] fn check_alpha_uppercase_test() {

    // This should be alpha uppercase:
    let s1 = String::from("AAAA");

    // These should not be:
    let has_lower = String::from("AaAa");
    let has_ws1 = String::from("A AA");
    let has_ws2 = String::from("AAA ");
    let not_ascii = String::from("aaaă");

    // Test correct behavior:
    let _ = check_alpha_uppercase(&s1, "s1").expect("s1 is compliant");

    // Test error behavior:
    let error = check_alpha_uppercase(&has_lower, "has_lower").err()
        .expect("has_lower is error");
    assert_eq!(
        error, 
        BadElementError::NotUppercaseAlpha(String::from("has_lower"))
    );

    let _ = check_alpha_uppercase(&has_ws1, "has_ws1").err()
        .expect("has_ws1 is error");
    let _ = check_alpha_uppercase(&has_ws2, "has_ws2").err()
        .expect("has_ws2 is error");
    let _ = check_alpha_uppercase(&not_ascii, "not_ascii").err()
        .expect("not_ascii is error");
}

use crate::helper::{
    fixed_str_4,
    fixed_str_8,
    fixed_str_14,
};

#[test] fn fixed_str_test() {
    
    let s = String::from("A");
    let s4 = String::from("AAAA");
    let s8 = String::from("AAAAAAAA");
    let s14 = String::from("AAAAAAAAAAAAAA");
    let s15 = String::from("AAAAAAAAAAAAAAA");

    // Test fixed_str_4
    assert_eq!(fixed_str_4(&s), *b"A   ");
    assert_eq!(fixed_str_4(&s4), [b'A'; 4]);
    assert_eq!(fixed_str_4(&s8), [b'A'; 4]);

    // Test fixed_str_8
    assert_eq!(fixed_str_8(&s4), *b"AAAA    ");
    assert_eq!(fixed_str_8(&s8), [b'A'; 8]);
    assert_eq!(fixed_str_8(&s14), [b'A'; 8]);

    // Test fixed_str_14
    assert_eq!(fixed_str_14(&s8), *b"AAAAAAAA      ");
    assert_eq!(fixed_str_14(&s14), [b'A'; 14]);
    assert_eq!(fixed_str_14(&s15), [b'A'; 14]);
}

use crate::helper::ascii_from_utf8;

#[test] fn from_utf8_test() {

    // Test correct behavior:
    let bytes = b"1Aa ";
    let ascii = ascii_from_utf8(bytes)
        .expect("'1Aa ' is valid ASCII")
        .to_string();

    assert_eq!(ascii, String::from("1Aa "));

    // TODO: Test error bahavior?
}

use crate::helper::{
    u16_from_be_bytes,
    u32_from_be_bytes,
    u64_from_be_bytes,
};

#[test] fn from_be_bytes_test() {

    // Test correct behavior:
    let bytes_a = u16::MIN.to_be_bytes();
    let bytes_b = 69u16.to_be_bytes();
    let bytes_c = u16::MAX.to_be_bytes();

    let _a = u16_from_be_bytes(&bytes_a)
        .expect("Valid u16");
    let _b = u16_from_be_bytes(&bytes_b)
        .expect("Valid u16");
    let _c = u16_from_be_bytes(&bytes_c)
        .expect("Valid u16");

    let bytes_a = u32::MIN.to_be_bytes();
    let bytes_b = 69u32.to_be_bytes();
    let bytes_c = u32::MAX.to_be_bytes();

    let _a = u32_from_be_bytes(&bytes_a)
        .expect("Valid u32");
    let _b = u32_from_be_bytes(&bytes_b)
        .expect("Valid u32");
    let _c = u32_from_be_bytes(&bytes_c)
        .expect("Valid u32");

    let bytes_a = u64::MIN.to_be_bytes();
    let bytes_b = 69u64.to_be_bytes();
    let bytes_c = u64::MAX.to_be_bytes();

    let _a = u64_from_be_bytes(&bytes_a)
        .expect("Valid u64");
    let _b = u64_from_be_bytes(&bytes_b)
        .expect("Valid u64");
    let _c = u64_from_be_bytes(&bytes_c)
        .expect("Valid u64");

    // More correct behavior:
    let bytes_a = b"zz";
    let _ = u16_from_be_bytes(bytes_a)
        .expect("Valid u16");
    let bytes_b = b"zzzz";
    let _ = u32_from_be_bytes(bytes_b)
        .expect("Valid u32");
    let bytes_c = b"zzzzzzzz";
    let _ = u64_from_be_bytes(bytes_c)
        .expect("Valid u64");

    // Test error behavior:
    let bad_a = b"zzz";
    let error_a = u16_from_be_bytes(bad_a).err()
        .expect("Should be invalid u16");
    assert_eq!(
        error_a,
        BadElementError::WrongSize(String::from("u16"), 2, 3)
    );

    let bad_b = b"zzzzz";
    let error_b = u32_from_be_bytes(bad_b).err()
        .expect("Should be invalid u32");
    assert_eq!(
        error_b,
        BadElementError::WrongSize(String::from("u32"), 4, 5)
    );

    let bad_c = b"zzzzzzzzz";
    let error_c = u64_from_be_bytes(bad_c).err()
        .expect("Should be invalid u64");
    assert_eq!(
        error_c,
        BadElementError::WrongSize(String::from("u64"), 8, 9)
    );
}

use crate::helper::nanosec_from_midnight;

#[test] fn naivetime_test() {
    // TODO: Test that nanosec_from_midnight always works
    // 86400 secs? times 10e9?
}

