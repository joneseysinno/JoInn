//! A round-trip law: one variable, self and one cell, nothing else.

use joinn_dna::{Formula, Term_, VarId};
use joinn_frame::Hash;

/// `(partner, self out, self in, partner out, partner in)` when the law is one of the two shapes.
pub(super) fn roundtrip(formula: &Formula) -> Option<(Hash, u32, u32, u32, u32)> {
    let Formula::ForAll { vars, body } = formula else {
        return None;
    };
    if vars.len() != 1 {
        return None;
    }
    let VarId(var) = &vars[0].0;
    let Formula::Eq(left, right) = body.as_ref() else {
        return None;
    };
    let is_var = |term: &Term_| matches!(term, Term_::Var(VarId(name)) if name == var);
    let cell_at = |term: &Term_| -> Option<(Hash, u32, u32)> {
        let Term_::CellAt { cell, out, args } = term else {
            return None;
        };
        if args.len() != 1 {
            return None;
        }
        let (port, inner) = args.iter().next()?;
        if !is_var(inner) {
            return None;
        }
        Some((*cell, *out, *port))
    };
    let self_at = |term: &Term_| -> Option<(u32, u32, Hash, u32, u32)> {
        let Term_::SelfAt { out, args } = term else {
            return None;
        };
        if args.len() != 1 {
            return None;
        }
        let (port, inner) = args.iter().next()?;
        let (cell, cell_out, cell_in) = cell_at(inner)?;
        Some((*out, *port, cell, cell_out, cell_in))
    };
    let cell_self = |term: &Term_| -> Option<(Hash, u32, u32, u32, u32)> {
        let Term_::CellAt { cell, out, args } = term else {
            return None;
        };
        if args.len() != 1 {
            return None;
        }
        let (port, inner) = args.iter().next()?;
        let Term_::SelfAt {
            out: self_out,
            args: self_args,
        } = inner
        else {
            return None;
        };
        if self_args.len() != 1 {
            return None;
        }
        let (self_in, inner_var) = self_args.iter().next()?;
        if !is_var(inner_var) {
            return None;
        }
        Some((*cell, *self_out, *self_in, *out, *port))
    };
    if is_var(right) {
        if let Some((self_out, self_in, cell, cell_out, cell_in)) = self_at(left) {
            return Some((cell, self_out, self_in, cell_out, cell_in));
        }
        if let Some(found) = cell_self(left) {
            return Some(found);
        }
    }
    if is_var(left) {
        if let Some((self_out, self_in, cell, cell_out, cell_in)) = self_at(right) {
            return Some((cell, self_out, self_in, cell_out, cell_in));
        }
        if let Some(found) = cell_self(right) {
            return Some(found);
        }
    }
    None
}
