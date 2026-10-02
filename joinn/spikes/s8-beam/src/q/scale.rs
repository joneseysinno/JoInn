use num_rational::BigRational;

use super::Q;

/// Scales by a plain count in ℚ (½, ⅙, the 12 of a unit). The tag is unchanged.
pub fn scale(a: &Q, n: &BigRational) -> Q {
    Q {
        value: &a.value * n,
        tag: a.tag,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac};
    use super::scale;
    use crate::tag::Tag;

    #[test]
    fn scale_multiplies_the_value_and_keeps_the_tag() {
        let span = Q::new(frac!(288, 1), Tag::PLACE_X);
        assert_eq!(
            scale(&span, &frac!(1, 2)),
            Q::new(frac!(144, 1), Tag::PLACE_X)
        );
        let moment = Q::new(frac!(5184, 5), Tag::MOMENT);
        assert_eq!(
            scale(&moment, &frac!(1, 12)),
            Q::new(frac!(432, 5), Tag::MOMENT)
        );
    }
}
