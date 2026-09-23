//! Truncate a control artifact after its first line.

/// Bytes with everything after the first line removed.
/// A one-line file becomes empty: there is no second line to keep.
pub(crate) fn damage_bytes(bytes: &[u8]) -> Vec<u8> {
    match bytes.iter().position(|b| *b == b'\n') {
        Some(i) if i + 1 < bytes.len() => bytes[..=i].to_vec(),
        _ => Vec::new(),
    }
}
