//! `cargo xtask codex`.

use std::io::{self, Write};

use super::codex_text;

/// Print codex 1 whole; any table check that fails is an error.
pub(crate) fn codex() -> Result<(), String> {
    let text = codex_text()?;
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
