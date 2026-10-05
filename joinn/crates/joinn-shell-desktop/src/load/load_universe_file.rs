//! Read and parse one `.universe` file.

use joinn_frame::Verdict;
use joinn_link::{Universe, parse_universe};
use std::fs;
use std::path::Path;

/// The parsed universe, or the path and the refusal.
pub(crate) fn load_universe_file(path: &Path) -> Result<Universe, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    match parse_universe(&text) {
        Verdict::Ok(u) => Ok(u),
        Verdict::Refused(r) => Err(format!("{}: {}", path.display(), r.reason)),
    }
}
