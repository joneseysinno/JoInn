use num_rational::BigRational;
use num_traits::Zero;

use super::Q;
use crate::verdict::{Verdict, refuse};

/// The plain count a / b, from two quantities with one tag. This is the one
/// exact linear solve: an unknown is a count times a unit of its tag.
pub fn ratio(a: &Q, b: &Q) -> Verdict<BigRational> {
    if a.tag != b.tag {
        return refuse(format!(
            "ratio: {} and {} differ; acceptance is two quantities with one tag",
            a.tag, b.tag
        ));
    }
    if b.value.is_zero() {
        return refuse(format!(
            "ratio: the divisor {} is 0; acceptance is a nonzero divisor",
            b.tag
        ));
    }
    Verdict::Admitted(&a.value / &b.value)
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac};
    use super::ratio;
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};

    #[test]
    fn a_ratio_of_one_tag_is_a_plain_count() {
        let a = Q::new(frac!(-4320, 1), Tag::MOMENT);
        let b = Q::new(frac!(288, 1), Tag::MOMENT);
        assert_eq!(admitted(ratio(&a, &b)), frac!(-15, 1));
    }

    #[test]
    fn a_ratio_of_two_tags_or_over_zero_is_refused() {
        let m = Q::new(frac!(1, 1), Tag::MOMENT);
        let w = Q::new(frac!(1, 1), Tag::WORK);
        assert_eq!(
            refused(ratio(&m, &w)),
            "ratio: source · length 1 · plane and energy · length 1 · none differ; acceptance is two quantities with one tag"
        );
        let zero = Q::new(frac!(0, 1), Tag::MOMENT);
        assert_eq!(
            refused(ratio(&m, &zero)),
            "ratio: the divisor source · length 1 · plane is 0; acceptance is a nonzero divisor"
        );
    }
}
