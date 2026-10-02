use super::Deflected;
use crate::q::{Q, frac, scale};
use crate::show::fraction;
use crate::verdict::{Verdict, refuse};

/// θ and v at x. A section is always a point of the complex.
pub fn deflected_at<'a>(deflected: &'a [Deflected], x: &Q) -> Verdict<&'a Deflected> {
    match deflected.iter().find(|d| &d.x == x) {
        Some(d) => Verdict::Admitted(d),
        None => refuse(format!(
            "section: {} ft is not a point of the complex; acceptance is a printed section, which the complex always holds",
            fraction(scale(x, &frac!(1, 12)).value())
        )),
    }
}
