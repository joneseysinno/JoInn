//! Gate 5 item 7 control: before revoke, units fires.

use super::{artifact_loads, units_after};

pub(crate) fn g5_revoke_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    match units_after(text, true) {
        Ok((1, Some(value))) if value == "60" => false,
        _ => true,
    }
}
