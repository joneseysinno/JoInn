//! Look up a regulatory prompt string.

/// Look up a regulatory prompt string.
pub(crate) fn prompt<'a>(body: &'a joinn_dna::Body, inst: &str, fallback: &'a str) -> &'a str {
    body.regulatory
        .prompts
        .get(inst)
        .map(String::as_str)
        .unwrap_or(fallback)
}
