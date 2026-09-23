//! Canonical print of a text term.

use crate::term::Term;

pub fn print_term(term: &Term) -> String {
    match term {
        Term::Text(s) => super::quote(s),
        _ => String::new(),
    }
}
