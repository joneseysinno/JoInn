//! The CLI's signal set is the environment body's genome.

use joinn_frame::{FrameRegistry, Verdict};
use joinn_host::Signals;
use std::fs;

use super::find_corpus::find_corpus;

/// Load `corpus/phase3/environment.body` and emit its instance names.
pub(crate) fn environment_signals() -> Result<Signals, String> {
    let path = find_corpus()?.join("phase3").join("environment.body");
    let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let body = match joinn_dna::parse_body(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    Ok(joinn_host::signals_from_environment(&body))
}
