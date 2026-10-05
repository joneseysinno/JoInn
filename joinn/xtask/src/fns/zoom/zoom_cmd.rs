//! `cargo xtask zoom [--measure]`.

use std::io::{self, Write};

use super::{zoom_measure, zoom_text};

/// Print the cut lines, the touches and the last line; with `--measure`, the
/// cut's cost per view instead. Any refusal is an error.
pub(crate) fn zoom(args: Vec<String>) -> Result<(), String> {
    let text = match args.as_slice() {
        [] => zoom_text()?,
        [flag] if flag == "--measure" => zoom_measure()?,
        _ => {
            return Err(format!(
                "zoom: arguments {args:?}; acceptance is none, or --measure"
            ));
        }
    };
    io::stdout()
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())
}
