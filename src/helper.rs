
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

