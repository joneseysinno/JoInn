use super::Q;
use crate::verdict::{Verdict, refuse};

/// Adds two quantities on one piece, side and pair. Equal units are not
/// enough: the tags must be equal.
pub fn add(a: &Q, b: &Q) -> Verdict<Q> {
    if a.tag != b.tag {
        let (unit_a, unit_b) = (a.tag.unit(), b.tag.unit());
        let though = if unit_a == unit_b {
            format!(", though both are {unit_a}")
        } else {
            String::new()
        };
        return refuse(format!(
            "add: {} and {} differ{though}; acceptance is two quantities on one piece, side and pair",
            a.tag, b.tag
        ));
    }
    Verdict::Admitted(Q {
        value: &a.value + &b.value,
        tag: a.tag,
    })
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac};
    use super::add;
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};

    #[test]
    fn equal_tags_add_their_values() {
        let a = Q::new(frac!(432, 5), Tag::MOMENT);
        let b = Q::new(frac!(-72, 5), Tag::MOMENT);
        let sum = admitted(add(&a, &b));
        assert_eq!(sum, Q::new(frac!(72, 1), Tag::MOMENT));
    }

    #[test]
    fn moment_plus_work_is_refused_though_both_are_kip_in() {
        let moment = Q::new(frac!(5184, 5), Tag::MOMENT);
        let work = Q::new(frac!(103680, 12325), Tag::WORK);
        assert_eq!(moment.tag().unit(), work.tag().unit());
        assert_eq!(
            refused(add(&moment, &work)),
            "add: source · length 1 · plane and energy · length 1 · none differ, though both are kip·in; acceptance is two quantities on one piece, side and pair"
        );
    }

    #[test]
    fn different_units_are_refused_without_the_though_clause() {
        let force = Q::new(frac!(10, 1), Tag::FORCE);
        let span = Q::new(frac!(288, 1), Tag::PLACE_X);
        assert_eq!(
            refused(add(&force, &span)),
            "add: source · length 0 · y and placement · length 1 · x differ; acceptance is two quantities on one piece, side and pair"
        );
    }
}
