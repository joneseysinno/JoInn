//! `cargo xtask strand <file>`.

use std::io::{self, Write};

use super::strand_text;

/// Print the strand; a letter that isn't A, T, G or C is an error.
pub(crate) fn strand(args: Vec<String>) -> Result<(), String> {
    let [path] = args.as_slice() else {
        return Err(format!(
            "strand: arguments {args:?}; acceptance is one strand file"
        ));
    };
    let text = strand_text(path)?;
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
