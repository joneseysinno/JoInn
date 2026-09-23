//! Gate 5.1 item 6 control: the pre-revoke run, read from the artifact.

use super::g5_revoke_control;

pub(crate) fn g51_revoke_control(art: &joinn_gate::Artifact) -> bool {
    g5_revoke_control(art)
}
