use num_rational::BigRational;
use num_traits::One;

/// `432/5`, `60`, `-3/2`: lowest terms, no decimal.
pub fn fraction(r: &BigRational) -> String {
    if r.denom().is_one() {
        r.numer().to_string()
    } else {
        format!("{}/{}", r.numer(), r.denom())
    }
}

#[cfg(test)]
mod tests {
    use super::fraction;
    use crate::q::frac;

    #[test]
    fn fractions_print_in_lowest_terms() {
        assert_eq!(fraction(&frac!(864, 10)), "432/5");
        assert_eq!(fraction(&frac!(120, 2)), "60");
        assert_eq!(fraction(&frac!(-3, 2)), "-3/2");
        assert_eq!(fraction(&frac!(0, 7)), "0");
    }
}
