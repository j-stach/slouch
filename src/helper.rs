
use chrono::NaiveTime;
use crate::error::BadElementError;


// `Alpha` fields may contain only upper- and lowercase letters,
// and must fit within the alloted fixed length. 
pub(crate) fn check_alpha_compliance(
    s: &str,
    len: usize,
    name: &str
) -> Result<(), BadElementError> {

    if s.len() > len  {
        return Err(
            BadElementError::WrongSize(name.to_string(), len, s.len())
        );
    }

    if !s.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(
            BadElementError::NotAlpha(name.to_string())
        );
    }

    Ok(())
}

// Used in fields where uppercase letters are required (e.g., `FirmId`)
pub(crate) fn check_alpha_uppercase(
    s: &str,
    name: &str
) -> Result<(), BadElementError> {
    
    if !s.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(
            BadElementError::NotUppercaseAlpha(name.to_string())
        );
    }

    Ok(())
}

// Store fixed string with length 4 by padding to the right with spaces.
pub(crate) fn fixed_str_4(s: &str) -> [u8; 4] {
    let mut buf = [b' '; 4];
    let chars = s.as_bytes();
    let copy_len = usize::min(4, chars.len());
    buf[..copy_len].copy_from_slice(&chars[..copy_len]);
    buf
}

// Store fixed string with length 8 by padding to the right with spaces.
pub(crate) fn fixed_str_8(s: &str) -> [u8; 8] {
    let mut buf = [b' '; 8];
    let chars = s.as_bytes();
    let copy_len = usize::min(8, chars.len());
    buf[..copy_len].copy_from_slice(&chars[..copy_len]);
    buf
}

// Store fixed string with length 14 by padding to the right with spaces.
pub(crate) fn fixed_str_14(s: &str) -> [u8; 14] {
    let mut buf = [b' '; 14];
    let chars = s.as_bytes();
    let copy_len = usize::min(14, chars.len());
    buf[..copy_len].copy_from_slice(&chars[..copy_len]);
    buf
}

// Simplifies conversion.
pub(crate) fn ascii_from_utf8(data: &[u8]) -> Result<String, BadElementError> {
    Ok(std::str::from_utf8(data)?.trim_end().to_string())
}

// Simplifies conversion.
pub(crate) fn u64_from_be_bytes(data: &[u8]) -> Result<u64, BadElementError> {

    if let Ok(bytes) = data.try_into() {
        Ok(u64::from_be_bytes(bytes))
    } else {
        Err(BadElementError::WrongSize("u64".to_string(), 8, data.len()))
    }
}

// Simplifies conversion.
pub(crate) fn u32_from_be_bytes(data: &[u8]) -> Result<u32, BadElementError> {

    if let Ok(bytes) = data.try_into() {
        Ok(u32::from_be_bytes(bytes))
    } else {
        Err(BadElementError::WrongSize("u32".to_string(), 4, data.len()))
    }
}

// Simplifies conversion.
pub(crate) fn u16_from_be_bytes(data: &[u8]) -> Result<u16, BadElementError> {

    if let Ok(bytes) = data.try_into() {
        Ok(u16::from_be_bytes(bytes))
    } else {
        Err(BadElementError::WrongSize("u16".to_string(), 2, data.len()))
    }
}

// Simplifies conversion.
pub(crate) fn i32_from_be_bytes(data: &[u8]) -> Result<i32, BadElementError> {

    if let Ok(bytes) = data.try_into() {
        Ok(i32::from_be_bytes(bytes))
    } else {
        Err(BadElementError::WrongSize("i32".to_string(), 4, data.len()))
    }
}

// Convert from encoded timestamp to Rust-friendly type.
pub(crate) fn nanosec_from_midnight(time: u64) -> NaiveTime {

    let d = 10u64.pow(9);
    let secs = (time / d) as u32;
    let nano = (time % d) as u32;

    // TODO: TBD: Is this safe to expect?
    // It is the same convention used by OUCH, so it should not be invalid.
    NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time")
} 

