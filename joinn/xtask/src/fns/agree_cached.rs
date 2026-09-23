//! One `agree()` result per process. Gate 2.2 items 1, 2 and 8 share it.

use std::sync::OnceLock;

use super::agree;

static CACHED: OnceLock<Result<(), String>> = OnceLock::new();

/// Run `agree` once and reuse the result.
pub(crate) fn agree_cached() -> Result<(), String> {
    match CACHED.get_or_init(agree) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.clone()),
    }
}
