//! Distinct pointer for the phase 2 transcript control.

use super::p21_transcript_control;

pub(crate) fn g2_transcript_control(art: &joinn_gate::Artifact) -> bool {
    p21_transcript_control(art)
}
