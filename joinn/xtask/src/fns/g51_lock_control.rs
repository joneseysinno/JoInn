//! Gate 5.1 item 9 control: the artifact lock is not a full pass.

use super::{artifact_loads, parse_lock_scores};

pub(crate) fn g51_lock_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(text) = std::str::from_utf8(art.bytes) else {
        return true;
    };
    match parse_lock_scores(text) {
        Ok(rows) => !rows.iter().any(|row| row.n != row.total),
        Err(_) => true,
    }
}
