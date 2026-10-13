//! Read the `.strand` file the shell was given.

use std::fs;
use std::path::Path;

use joinn_frame::Verdict;
use joinn_strand::{StrandFile, parse_strand_file};

/// The strand file as `joinn-strand` parses it; a refusal is the parser's.
pub(crate) fn load_strand_file(path: &Path) -> Result<StrandFile, String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    match parse_strand_file(&src) {
        Verdict::Ok(f) => Ok(f),
        Verdict::Refused(r) => Err(r.reason),
    }
}
