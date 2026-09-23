//! Gate 5 item 1 control: without e0, units does not fire.

use super::{artifact_loads, units_after};

pub(crate) fn g5_linked_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    match units_after(text, false) {
        Ok((0, _)) => false,
        _ => true,
    }
}
