//! Print a position→term argument map.

use crate::formula::Term_;
use crate::print::print_term_::print_term_;

/// Print a position→term argument map.
pub(in crate::print) fn print_args(args: &std::collections::BTreeMap<u32, Term_>) -> String {
    args.iter()
        .map(|(p, t)| format!("{p}: {}", print_term_(t)))
        .collect::<Vec<_>>()
        .join(", ")
}
