//! Distinct pointer for the phase 2 corpus control.

use super::p22_corpus_control;

pub(crate) fn g2_corpus_control(art: &joinn_gate::Artifact) -> bool {
    p22_corpus_control(art)
}
