//! Parse the `.body` file the shell was given.

use joinn_dna::parse_body;
use joinn_frame::{FrameRegistry, Verdict};
use std::fs;
use std::path::Path;

/// Parse `path` as a body. The argument is a path, not a stem.
pub(crate) fn load_body_file(path: &Path) -> Result<joinn_dna::Body, String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    match parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(body) => Ok(body),
        Verdict::Refused(r) => Err(r.reason),
    }
}
