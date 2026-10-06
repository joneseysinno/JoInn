//! Combine's identity, read from the response cell's own law.

use joinn_dna::{Cell, Formula, Term_};
use joinn_frame::{FrameRegistry, Value, Verdict};

/// The constant `e` of the response's identity law
/// `forall a. self@out(…: a, …: e) = a`, built by its frame. An empty body
/// presents it; no cell and no register row hold it.
pub(crate) fn identity(cell: &Cell, frames: &FrameRegistry) -> Verdict<Value> {
    for law in cell.coding.laws.values() {
        let Formula::ForAll { vars, body } = &law.formula else {
            continue;
        };
        let [(a, _)] = vars.as_slice() else {
            continue;
        };
        let Formula::Eq(Term_::SelfAt { args, .. }, Term_::Var(rhs)) = body.as_ref() else {
            continue;
        };
        if rhs != a || args.len() != 2 {
            continue;
        }
        let mut terms = args.values();
        let (Some(x), Some(y)) = (terms.next(), terms.next()) else {
            continue;
        };
        let constant = match (x, y) {
            (Term_::Var(v), c) | (c, Term_::Var(v)) if v == a => c,
            _ => continue,
        };
        let Term_::FrameOp { frame, op, args } = constant else {
            continue;
        };
        if !args.is_empty() {
            continue;
        }
        let Some(f) = frames.get(frame) else {
            continue;
        };
        return f.apply_op(op, &[]);
    }
    crate::refuse(
        "grow: the response has no identity law; acceptance is forall a. self(a, e) = a for a constant e",
    )
}
