//! Scan crates/ and xtask/src for module-layout violations.

mod classify;
mod is_enforced;
mod is_trait_impl_only;
mod is_under_tests_dir;
mod load_lines;
mod run;
mod walk_rs;
mod workspace_root;

pub use classify::{FileKind, classify};
pub use run::run;
