//! Gate 3 item 8 control: the fixture artifact must still be missing.

use super::{fixture_line, resolve_named};

pub(crate) fn g3_artifacts_control(_art: &()) -> bool {
    let Ok((name, rel)) = fixture_line() else {
        return true;
    };
    resolve_named(1, &name, &rel).is_ok()
}
