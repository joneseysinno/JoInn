//! Gate 3 item 8 control: the fixture artifact must still be missing.

use super::{artifact_loads, fixture_line, resolve_named};

pub(crate) fn g3_artifacts_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok((name, rel)) = fixture_line() else {
        return true;
    };
    resolve_named(1, &name, &rel).is_ok()
}
