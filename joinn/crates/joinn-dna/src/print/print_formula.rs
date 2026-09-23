//! Canonical formula text.

use crate::formula::Formula;
use crate::print::print_term_::print_term_;

/// Canonical formula text.
pub fn print_formula(f: &Formula) -> String {
    match f {
        Formula::Eq(a, b) => format!("{} = {}", print_term_(a), print_term_(b)),
        Formula::Not(inner) => format!("¬ ({})", print_formula(inner)),
        Formula::And(xs) => {
            let parts: Vec<String> = xs
                .iter()
                .map(|x| format!("({})", print_formula(x)))
                .collect();
            format!("∧ {}", parts.join(" "))
        }
        Formula::Or(xs) => {
            let parts: Vec<String> = xs
                .iter()
                .map(|x| format!("({})", print_formula(x)))
                .collect();
            format!("∨ {}", parts.join(" "))
        }
        Formula::Implies(a, b) => format!("({}) → ({})", print_formula(a), print_formula(b)),
        Formula::ForAll { vars, body } => {
            let mut binders: Vec<&(crate::formula::VarId, joinn_frame::FrameRef)> =
                vars.iter().collect();
            binders.sort_by(|x, y| x.0.cmp(&y.0));
            let mut s = String::from("forall");
            for (v, fr) in binders {
                s.push(' ');
                s.push_str(&v.0);
                s.push(':');
                s.push_str(&fr.to_string());
            }
            s.push_str(". ");
            s.push_str(&print_formula(body));
            s
        }
    }
}
