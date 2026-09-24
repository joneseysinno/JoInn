//! Detect a function declaration after visibility/async prefixes.

/// True when `trimmed` starts a named `fn` item.
pub fn is_fn_decl(trimmed: &str) -> bool {
    let t = trimmed
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("pub(super) ")
        .trim_start_matches("pub ")
        .trim_start_matches("async ")
        .trim_start_matches("const ")
        .trim_start_matches("unsafe ");
    t.starts_with("fn ") || t.starts_with("fn\t")
}
