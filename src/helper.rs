
// Helper function
pub(crate) fn encode_fixed_str(s: &str, len: usize) -> Vec<u8> {
    let mut buf = vec![b' '; len];
    let bytes = s.as_bytes();
    let copy_len = usize::min(len, bytes.len());
    buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
    buf
}

