use num_rational::BigRational;
use num_traits::Signed;

use super::{Kind, Sense, Witness};
use crate::q::Q;
use crate::show::fraction;
use crate::verdict::{Verdict, refuse};

/// A witness agrees when the derived value's magnitude equals it exactly and
/// the derived sense is the case's. A disagreement is a truth violation.
pub fn compare(id: &str, witness: &Witness, derived: &Q) -> Verdict<()> {
    let value = derived.value();
    let (sense, shown, unit): (Sense, BigRational, &str) = match witness.kind {
        Kind::Moment => (
            if value.is_negative() {
                Sense::Hogging
            } else {
                Sense::Sagging
            },
            value.abs() / BigRational::from_integer(12.into()),
            "kip·ft",
        ),
        Kind::Deflection => (
            if value.is_positive() {
                Sense::Up
            } else {
                Sense::Down
            },
            value.abs(),
            "in",
        ),
    };
    let stated = match witness.kind {
        Kind::Moment => &witness.value / BigRational::from_integer(12.into()),
        Kind::Deflection => witness.value.clone(),
    };
    if shown == stated && sense == witness.sense {
        return Verdict::Admitted(());
    }
    let senses = if sense == witness.sense {
        (String::new(), String::new())
    } else {
        (format!(" {}", witness.sense), format!(" {sense}"))
    };
    refuse(format!(
        "{id} witness {} = {} {unit}{}, derived {} {unit}{}; a disagreement is a truth violation",
        witness.name,
        fraction(&stated),
        senses.0,
        fraction(&shown),
        senses.1
    ))
}

#[cfg(test)]
mod tests {
    use super::compare;
    use crate::q::{Q, frac};
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};
    use crate::witness::{Kind, Sense, Witness};

    fn at(x: i64) -> Q {
        Q::new(frac!(x, 1), Tag::PLACE_X)
    }

    #[test]
    fn an_equal_magnitude_with_the_case_sense_agrees() {
        let w = Witness::new(
            "PL".to_string(),
            at(0),
            Kind::Moment,
            Sense::Hogging,
            frac!(600, 1),
        );
        admitted(compare("E4", &w, &Q::new(frac!(-600, 1), Tag::MOMENT)));
    }

    #[test]
    fn a_different_value_or_sense_is_a_truth_violation() {
        let w = Witness::new(
            "wL²/12".to_string(),
            at(144),
            Kind::Moment,
            Sense::Sagging,
            frac!(3456, 5),
        );
        assert_eq!(
            refused(compare("E1", &w, &Q::new(frac!(5184, 5), Tag::MOMENT))),
            "E1 witness wL²/12 = 288/5 kip·ft, derived 432/5 kip·ft; a disagreement is a truth violation"
        );
        let v = Witness::new(
            "PL³/3EI".to_string(),
            at(120),
            Kind::Deflection,
            Sense::Down,
            frac!(240, 493),
        );
        assert_eq!(
            refused(compare("E4", &v, &Q::new(frac!(240, 493), Tag::DEFLECTION))),
            "E4 witness PL³/3EI = 240/493 in down, derived 240/493 in up; a disagreement is a truth violation"
        );
    }
}
