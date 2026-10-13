//! `cargo xtask read <file>`.

use std::io::{self, Write};

use super::read_text;

/// Print the proteins and the partner's; a refusal is an error.
pub(crate) fn read(args: Vec<String>) -> Result<(), String> {
    let [path] = args.as_slice() else {
        return Err(format!(
            "read: arguments {args:?}; acceptance is one strand file"
        ));
    };
    let text = read_text(path)?;
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
