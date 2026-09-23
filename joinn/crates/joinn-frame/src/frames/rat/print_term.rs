//! Canonical print for ℚ.

use crate::term::Term;

pub fn print_term(term: &Term) -> String {
    match super::as_ratio(term) {
        Some(r) => format!("{}/{}", r.numer(), r.denom()),
        None => String::new(),
    }
}
