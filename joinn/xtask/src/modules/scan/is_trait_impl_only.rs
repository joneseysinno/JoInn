//! True when the file's production body is only `impl SomeTrait for T { ... }`.

/// True when the file's production body is only `impl SomeTrait for T { ... }`.
pub fn is_trait_impl_only(text: &str) -> bool {
    let stripped = super::super::count::strip_cfg_test_for_scan(text);
    let mut depth = 0i32;
    let mut saw_trait_impl = false;
    for line in stripped.lines() {
        let t = line.trim_start();
        let at_top = depth == 0;
        if at_top {
            if t.is_empty() || t.starts_with("//") || t.starts_with("use ") || t.starts_with("#!") {
                // fall through to brace count
            } else if t.starts_with("impl ") && t.contains(" for ") {
                saw_trait_impl = true;
            } else if t.starts_with("fn ")
                || t.starts_with("pub fn ")
                || t.starts_with("pub(crate) fn ")
                || t.starts_with("pub fn ")
                || (t.starts_with("impl ") && !t.contains(" for "))
                || t.starts_with("mod ")
            {
                // Free fns, inherent impls, and nested mods break the exception.
                // Accompanying type/const/static defs for the impl'd type are fine.
                return false;
            }
        }
        depth += line.chars().filter(|&c| c == '{').count() as i32;
        depth -= line.chars().filter(|&c| c == '}').count() as i32;
        if depth < 0 {
            depth = 0;
        }
    }
    saw_trait_impl
}
