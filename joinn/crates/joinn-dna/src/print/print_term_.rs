//! Canonical term text.

use crate::formula::Term_;
use crate::print::print_args::print_args;

/// Canonical term text.
pub fn print_term_(t: &Term_) -> String {
    match t {
        Term_::Var(v) => v.0.clone(),
        Term_::Lit(v) => v.print_literal(),
        Term_::FrameOp { frame, op, args } => {
            if args.is_empty() {
                format!("{}.{}", frame, op.as_str())
            } else {
                let inner: Vec<String> = args.iter().map(print_term_).collect();
                format!("{}.{}({})", frame, op.as_str(), inner.join(", "))
            }
        }
        Term_::SelfAt { out, args } => {
            format!("self@{out}({})", print_args(args))
        }
        Term_::CellAt { cell, out, args } => {
            format!("cell:{}@{out}({})", cell.to_hex(), print_args(args))
        }
    }
}
