//! Gate 5 item 5 control: the same pair inside one lens must stay refused.

use super::{artifact_loads, load_universe_file};
use joinn_frame::Verdict;
use joinn_link::check_lenses;

pub(crate) fn g5_lenses_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(u) = load_universe_file("phase5/controls/two_systems.universe") else {
        return true;
    };
    match check_lenses(&u) {
        Verdict::Ok(()) => true,
        Verdict::Refused(_) => false,
    }
}
