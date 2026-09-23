//! Gate 5 item 3 control: calc.sum@9 is no such port, not interior.

use super::{artifact_loads, load_phase5_bodies};
use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind_bodies, parse_universe};

pub(crate) fn g5_assemble_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    let universe = match parse_universe(text) {
        Verdict::Ok(u) => u,
        Verdict::Refused(_) => return true,
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return true;
    };
    let bound = match bind_bodies(&universe, &supplied) {
        Verdict::Ok(b) => b,
        Verdict::Refused(_) => return true,
    };
    match assemble_universe(&universe, &bound) {
        Verdict::Refused(r) => {
            !(r.reason.contains("calc.sum@9")
                && r.reason.contains("no such port")
                && !r.reason.contains("interior"))
        }
        Verdict::Ok(()) => true,
    }
}
