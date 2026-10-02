use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// The decimal a non-whole value prints beside its fraction: at most 4 places
/// with trailing zeros dropped when exact, or rounded to exactly 4 places
/// (half up, away from zero) with `~` when not. None for a whole number.
pub fn decimal(r: &BigRational) -> Option<String> {
    if r.is_integer() {
        return None;
    }
    let places = BigInt::from(10_000);
    let scaled = r.abs() * BigRational::from_integer(places.clone());
    let exact = scaled.is_integer();
    let half = BigRational::new(BigInt::from(1), BigInt::from(2));
    let n = if exact {
        scaled.to_integer()
    } else {
        (scaled + half).floor().to_integer()
    };
    let whole = &n / &places;
    let rest = (&n % &places).to_string();
    let mut digits = format!("{}{rest}", "0".repeat(4usize.saturating_sub(rest.len())));
    if exact {
        while digits.ends_with('0') {
            digits.pop();
        }
    }
    let sign = if r.is_negative() && !n.is_zero() {
        "-"
    } else {
        ""
    };
    let tilde = if exact { "" } else { "~" };
    Some(format!("{sign}{whole}.{digits}{tilde}"))
}

#[cfg(test)]
mod tests {
    use super::decimal;
    use crate::q::frac;

    #[test]
    fn exact_decimals_drop_trailing_zeros() {
        assert_eq!(decimal(&frac!(432, 5)).as_deref(), Some("86.4"));
        assert_eq!(decimal(&frac!(549, 5)).as_deref(), Some("109.8"));
        assert_eq!(decimal(&frac!(1, 4)).as_deref(), Some("0.25"));
        assert_eq!(decimal(&frac!(-3, 2)).as_deref(), Some("-1.5"));
    }

    #[test]
    fn inexact_decimals_round_half_up_to_four_places_with_a_tilde() {
        assert_eq!(decimal(&frac!(93312, 61625)).as_deref(), Some("1.5142~"));
        assert_eq!(decimal(&frac!(478224, 308125)).as_deref(), Some("1.5520~"));
        assert_eq!(decimal(&frac!(240, 493)).as_deref(), Some("0.4868~"));
        assert_eq!(decimal(&frac!(1, 32)).as_deref(), Some("0.0313~"));
        assert_eq!(decimal(&frac!(2, 3)).as_deref(), Some("0.6667~"));
        assert_eq!(decimal(&frac!(1, 3)).as_deref(), Some("0.3333~"));
    }

    #[test]
    fn whole_numbers_print_no_decimal() {
        assert_eq!(decimal(&frac!(60, 1)), None);
        assert_eq!(decimal(&frac!(0, 1)), None);
    }
}
