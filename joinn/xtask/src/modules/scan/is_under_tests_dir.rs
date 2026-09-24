//! True for crate-level integration/trybuild trees (`crates/*/tests/**`).

/// True for crate-level integration/trybuild trees (`crates/*/tests/**`).
pub fn is_under_tests_dir(rel: &str) -> bool {
    rel.contains("/tests/")
}
