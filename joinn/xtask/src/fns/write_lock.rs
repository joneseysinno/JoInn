//! Write a lock file from the scores each gate returned.

use std::fs;
use std::path::Path;

use super::lock_text::lock_text;

/// Writes only when the file's bytes would change: an unchanged lock keeps
/// its bytes and its timestamp. Returns whether it wrote.
pub(crate) fn write_lock(
    path: &Path,
    outcomes: &[(&str, bool, u32, u32, bool)],
) -> Result<bool, String> {
    let body = lock_text(outcomes);
    if fs::read(path).is_ok_and(|have| have == body.as_bytes()) {
        return Ok(false);
    }
    fs::write(path, body).map_err(|e| e.to_string())?;
    Ok(true)
}
