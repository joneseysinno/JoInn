//! Canonical decimal print for ℤ.

use crate::term::Term;

pub fn print_term(term: &Term) -> String {
    match term {
        Term::Int(n) => n.to_string(),
        _ => String::new(),
    }
}
