//! Count production `fn` items, ignoring `#[cfg(test)]` regions.

mod allow_modules_reason;
mod is_fn_decl;
mod line_has_allow_modules;
mod production_fns;
mod strip_cfg_test_for_scan;

pub use allow_modules_reason::allow_modules_reason;
pub use production_fns::production_fns;
pub use strip_cfg_test_for_scan::strip_cfg_test_for_scan;
