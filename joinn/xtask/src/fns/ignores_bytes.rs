//! A control that never looks at the bytes. The gate must refuse it.

pub(crate) fn ignores_bytes(_: &joinn_gate::Artifact) -> bool {
    false
}
