//! A control can see its opposition only when the artifact still parses.

use joinn_dna::{parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_gate::Artifact;
use joinn_link::parse_universe;

/// True when `art` is still the document the control judges.
pub(crate) fn artifact_loads(art: &Artifact) -> bool {
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return false;
    };
    if text.is_empty() {
        return false;
    }
    let frames = FrameRegistry::phase1();
    if art.path.ends_with(".body") {
        return matches!(parse_body(text, &frames), Verdict::Ok(_));
    }
    if art.path.ends_with(".cell") {
        return matches!(parse_cell(text, &frames), Verdict::Ok(_));
    }
    if art.path.ends_with(".universe") {
        return matches!(parse_universe(text), Verdict::Ok(_));
    }
    if art.path.ends_with(".lock") {
        return text.lines().any(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with('#') && line.contains(':')
        });
    }
    match text.find('\n') {
        Some(i) => !text[i + 1..].trim().is_empty(),
        None => true,
    }
}
