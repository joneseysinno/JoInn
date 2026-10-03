//! `cargo xtask zoom`.

use std::io::{self, Write};

use super::zoom_text;

/// Print the cut lines, the touches and the last line. Any refusal is an error.
pub(crate) fn zoom() -> Result<(), String> {
    let text = zoom_text()?;
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
