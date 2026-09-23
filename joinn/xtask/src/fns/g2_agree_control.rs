//! Distinct pointer for the phase 2 agree control.

use super::p21_agree_control;

pub(crate) fn g2_agree_control(art: &joinn_gate::Artifact) -> bool {
    p21_agree_control(art)
}
