//! Extract rational from ℚ value.

use joinn_frame::{FrameId, Term, Value};
use num_rational::BigRational;
use num_traits::Zero;

pub(crate) fn rat_of(v: &Value) -> Option<BigRational> {
    if v.frame().id != FrameId::Rat {
        return None;
    }
    match v.term() {
        Term::Seq(xs) if xs.len() == 2 => match (&xs[0], &xs[1]) {
            (Term::Int(n), Term::Int(d)) if !d.is_zero() => {
                Some(BigRational::new(n.clone(), d.clone()))
            }
            _ => None,
        },
        Term::Int(n) => Some(BigRational::from(n.clone())),
        _ => None,
    }
}
