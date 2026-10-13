//! `cargo xtask tower`.

use std::io::{self, Write};

use super::tower_text;

/// Print the tower whole; any fact that fails is an error.
pub(crate) fn tower() -> Result<(), String> {
    let text = tower_text()?;
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
