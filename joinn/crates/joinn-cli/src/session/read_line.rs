//! Read the next line from the iterator.

use std::io;

/// Read the next line from the iterator.
pub(in crate::session) fn read_line(
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<String, String> {
    match lines.next() {
        Some(Ok(s)) => Ok(s),
        Some(Err(e)) => Err(e.to_string()),
        None => Err("unexpected end of input".into()),
    }
}
