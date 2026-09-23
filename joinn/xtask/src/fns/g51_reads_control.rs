//! Gate 5.1 item 8 control: the fixture source is the ignoring control.

use super::artifact_loads;

pub(crate) fn g51_reads_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    !(text.contains("ignores_its_bytes") && text.contains("_art"))
}
