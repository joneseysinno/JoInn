//! Gate 5 item 4 control: a well-formed universe must still pass exclusivity.

use super::{artifact_loads, load_universe_file};
use joinn_frame::Verdict;
use joinn_link::check_lenses;

pub(crate) fn g5_exclusive_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(u) = load_universe_file("phase5/universe.universe") else {
        return true;
    };
    match check_lenses(&u) {
        Verdict::Ok(()) => false,
        Verdict::Refused(_) => true,
    }
}
