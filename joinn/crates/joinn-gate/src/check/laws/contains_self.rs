//! Walk a term for `SelfAt`.

use joinn_dna::Term_;

pub(in crate::check::laws) fn contains_self(t: &Term_) -> bool {
    match t {
        Term_::SelfAt { .. } => true,
        Term_::FrameOp { args, .. } => args.iter().any(contains_self),
        Term_::CellAt { args, .. } => args.values().any(contains_self),
        _ => false,
    }
}
