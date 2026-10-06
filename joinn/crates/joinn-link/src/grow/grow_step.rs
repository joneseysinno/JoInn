//! One growth step: the body accepts an input and grows the next instance.

use joinn_dna::Accept;
use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};

use super::Grown;

/// Check `input` against what the body accepts, then grow `numbers.<n>`
/// filled with it. The system is unchanged; only the input list grows.
pub fn grow_step(grown: &Grown, input: &Value) -> Verdict<Grown> {
    let Some(grows) = grown.contact.coding.grows.as_ref() else {
        return crate::refuse(
            "grow: the body does not grow; acceptance is a contact with a grows section",
        );
    };
    let Some(force) = grown.force() else {
        return crate::refuse("grow: the system has no force; acceptance is a force");
    };
    let word = match grows.accepts {
        Accept::One => "counting",
        Accept::Any => "adding",
    };
    if input.frame() != &force.frame {
        return crate::refuse(format!(
            "{word}: {} is not in {}; acceptance is a value in {}",
            input.print_literal(),
            force.frame,
            force.frame
        ));
    }
    if grows.accepts == Accept::One {
        let one = match IntFrame::new().canonicalize(Term::int(1)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if *input != one {
            return crate::refuse(format!(
                "{word}: {} is not one; acceptance is 1 (counting grows by one)",
                input.print_term()
            ));
        }
    }
    let mut next = grown.clone();
    next.inputs.push(input.clone());
    Verdict::Ok(next)
}
