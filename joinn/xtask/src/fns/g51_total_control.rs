//! Gate 5.1 item 5: a missing cell is refused by name.

use super::artifact_loads;
use joinn_dna::parse_body;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::membrane;
use std::collections::BTreeMap;

pub(crate) fn g51_total_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    let Verdict::Ok(body) = parse_body(text, &FrameRegistry::phase1()) else {
        return true;
    };
    match membrane(&body, &BTreeMap::new()) {
        Verdict::Refused(r) => !(r.reason.contains("scale") && r.reason.contains("12b6")),
        Verdict::Ok(_) => true,
    }
}
