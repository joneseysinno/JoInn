//! Whether a source path lies below the renderer boundary (rule 54).

/// Only joinn-gpu and joinn-shell-desktop may hold floats. `path` uses forward slashes.
pub(crate) fn float_allowed(path: &str) -> bool {
    path.contains("/crates/joinn-gpu/") || path.contains("/crates/joinn-shell-desktop/")
}
