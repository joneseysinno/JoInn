//! Parse a corpus-relative .universe file.

use super::workspace_root;
use joinn_frame::Verdict;
use joinn_link::{Universe, parse_universe};
use std::fs;

pub(crate) fn load_universe_file(rel: &str) -> Result<Universe, String> {
    let path = workspace_root()?.join("corpus").join(rel);
    let src = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    match parse_universe(&src) {
        Verdict::Ok(u) => Ok(u),
        Verdict::Refused(r) => Err(r.reason),
    }
}
