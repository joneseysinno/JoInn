//! Distinct pointer for the phase 2 trace control.

use super::p21_trace_control;

pub(crate) fn g2_trace_control(art: &joinn_gate::Artifact) -> bool {
    p21_trace_control(art)
}
