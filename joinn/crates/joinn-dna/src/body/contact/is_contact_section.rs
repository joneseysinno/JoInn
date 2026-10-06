//! `is_contact_section`.

/// A word that opens a contact section. `wires` and `declarations` are named so
/// a flat list ends before them and they are refused, not read as instances.
pub(in crate::body::contact) fn is_contact_section(word: &str) -> bool {
    matches!(
        word,
        "budget"
            | "codex"
            | "declarations"
            | "forces"
            | "genome"
            | "grants"
            | "grows"
            | "lineage"
            | "read"
            | "steps"
            | "wires"
    )
}
