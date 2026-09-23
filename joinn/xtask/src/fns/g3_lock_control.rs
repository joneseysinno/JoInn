//! Gate 3 item 9 control: bad.lock must be refused, naming phase 3.

use super::{artifact_loads, parse_lock_scores};

pub(crate) fn g3_lock_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    match parse_lock_scores(text) {
        Ok(rows) => {
            !rows
                .iter()
                .any(|row| row.phase.contains("phase 3") && row.n != row.total)
        }
        Err(_) => true,
    }
}
