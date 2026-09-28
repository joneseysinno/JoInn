//! Float fixtures: a float in joinn-visual is flagged; a float in joinn-gpu is not.

use super::{float_allowed, line_has, workspace_root};
use std::fs;

/// The float fence must flag its joinn-visual fixture and accept its joinn-gpu fixture.
pub(crate) fn check_float_fixtures() -> Result<(), String> {
    let root = workspace_root()?;
    let words = [concat!("f", "32"), concat!("f", "64")];
    let flagged = |rel: &str| -> Result<bool, String> {
        let text = fs::read_to_string(root.join(rel)).map_err(|e| format!("{rel}: {e}"))?;
        let path = root.join(rel).to_string_lossy().replace('\\', "/");
        if float_allowed(&path) {
            return Ok(false);
        }
        Ok(text
            .lines()
            .any(|line| words.iter().any(|w| line_has(line, w))))
    };
    if !flagged("xtask/vocab_fixtures/crates/joinn-visual/src/lib.rs")? {
        return Err(
            "vocab: blind float fence - the joinn-visual fixture's float must be flagged".into(),
        );
    }
    if flagged("xtask/vocab_fixtures/crates/joinn-gpu/src/lib.rs")? {
        return Err(
            "vocab: float fence refuses its accept fixture - joinn-gpu may hold floats".into(),
        );
    }
    Ok(())
}
