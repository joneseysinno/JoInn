//! Print one line and flush it, so a click's report is not stuck in a buffer.

use std::io::{self, Write};

/// One line on stdout, flushed.
pub(super) fn emit(line: &str) {
    println!("{line}");
    let _ = io::stdout().flush();
}
