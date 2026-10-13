//! `cargo xtask pool`.

use std::io::{self, Write};

use super::pool_text;

/// Print the pool whole; anything but the cycle's refusal is an error.
pub(crate) fn pool() -> Result<(), String> {
    let text = pool_text()?;
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
