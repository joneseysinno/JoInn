//! A universe named on the command line: `grove`, a path, or a corpus path.

use joinn_frame::Verdict;
use joinn_link::{Universe, parse_universe};
use std::fs;
use std::path::PathBuf;

use super::{GROVE_SEED, grow_grove};
use crate::fns::workspace_root;

/// `grove` grows seed 7; otherwise `arg` is read as a path, or failing that as
/// a path under corpus/.
pub(crate) fn load_universe_arg(arg: &str) -> Result<Universe, String> {
    if arg == "grove" {
        return grow_grove(GROVE_SEED);
    }
    let direct = PathBuf::from(arg);
    let path = if direct.is_file() {
        direct
    } else {
        let rel = arg.replace('\\', "/");
        let rel = rel.strip_prefix("corpus/").unwrap_or(&rel).to_owned();
        workspace_root()?.join("corpus").join(rel)
    };
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    match parse_universe(&text) {
        Verdict::Ok(u) => Ok(u),
        Verdict::Refused(r) => Err(format!("{}: {}", path.display(), r.reason)),
    }
}
