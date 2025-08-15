
use chrono::NaiveTime;

use crate::error::BadElementError;

pub(crate) fn check_string_compliance(
    s: &str,
    len: usize,
    name: &str
) -> Result<(), BadElementError> {

    if s.len() != len {
        return Err(
            BadElementError::WrongSize(name.to_string(), len, s.len())
        );
    }

    if !s.chars().all(|c| c.is_ascii()) {
        return Err(
            BadElementError::NotAscii(name.to_string())
        );
    }

    Ok(())

}

pub(crate) fn encode_fixed_str(s: &str, len: usize) -> Vec<u8> {
    let mut buf = vec![b' '; len];
    let bytes = s.as_bytes();
    let copy_len = usize::min(len, bytes.len());
    buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
    buf
}

pub(crate) fn u64_from_be_bytes(data: &[u8]) -> Result<u64, BadElementError> {

    if let Ok(bytes) = data.try_into() {
        Ok(u64::from_be_bytes(bytes))
    } else {
        Err(BadElementError::WrongSize("u64".to_string(), 8, data.len()))
    }
}

pub(crate) fn u32_from_be_bytes(data: &[u8]) -> Result<u32, BadElementError> {

    if let Ok(bytes) = data.try_into() {
        Ok(u32::from_be_bytes(bytes))
    } else {
        Err(BadElementError::WrongSize("u32".to_string(), 4, data.len()))
    }
}

pub(crate) fn nanosec_from_midnight(time: u64) -> NaiveTime {

    let d = 10u64.pow(9);
    let secs = (time / d) as u32;
    let nano = (time % d) as u32;

    // DEBUG: Is this safe to expect?
    NaiveTime::from_num_seconds_from_midnight_opt(secs, nano)
        .expect("Timestamp is a valid time")
} 
