//! Gate 5.1 item 1 control: the unlinked universe, read from the artifact.

use super::g5_linked_control;

pub(crate) fn g51_cross_control(art: &joinn_gate::Artifact) -> bool {
    g5_linked_control(art)
}
