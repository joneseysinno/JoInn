//! Read a strand file from disk.

use joinn_frame::Verdict;
use joinn_strand::{StrandFile, parse_strand_file};

/// The parsed file, or the path and the refusal.
pub(crate) fn read_strand_file(path: &str) -> Result<StrandFile, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    match parse_strand_file(&text) {
        Verdict::Ok(f) => Ok(f),
        Verdict::Refused(r) => Err(format!("{path}: {}", r.reason)),
    }
}
