//! Gate 5.1 item 7 control: the artifact's text is what the probe must reach.

use super::g5_locality_control;

pub(crate) fn g51_home_control(art: &joinn_gate::Artifact) -> bool {
    g5_locality_control(art)
}
