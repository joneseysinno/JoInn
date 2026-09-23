//! Locate the workspace `corpus/` directory.

use std::path::PathBuf;

/// Locate the workspace `corpus/` directory.
pub(crate) fn find_corpus() -> Result<PathBuf, String> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..6 {
        let c = p.join("corpus");
        if c.is_dir() {
            return Ok(c);
        }
        if !p.pop() {
            break;
        }
    }
    Err("corpus/ not found from joinn-cli".into())
}
