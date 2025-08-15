
use crate::error::BadElementError;

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

