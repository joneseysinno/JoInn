use num_rational::BigRational;
use num_traits::{Signed, Zero};

use super::{Kind, Sense, Witness};
use crate::beam::Example;
use crate::bridge::Edition;
use crate::q::{Q, frac, scale};
use crate::show::fraction;
use crate::tag::Tag;

/// Each example's witnesses, as AISC Table 3-23 and Roark Table 8.1 state
/// them, from the example's inputs (w, P, L, a, b) and the edition's E·I.
pub fn stated(example: &Example, edition: &Edition) -> Vec<Witness> {
    let l = example.span().value().clone();
    let ei = edition.e() * edition.ix();
    if l.is_zero() || ei.is_zero() {
        return Vec::new();
    }
    let w = example
        .uniform()
        .map(|q| q.value().abs())
        .unwrap_or_default();
    let (a, p) = example
        .point_loads()
        .first()
        .map(|load| (load.at().value().clone(), load.force().value().abs()))
        .unwrap_or_default();
    let b = &l - &a;
    let n = |k: i64| BigRational::from_integer(k.into());
    let at = |x: &BigRational| Q::new(x.clone(), Tag::PLACE_X);
    let mid = &l / n(2);
    let moment = |name: &str, x: &BigRational, sense: Sense, value: BigRational| {
        Witness::new(name.to_string(), at(x), Kind::Moment, sense, value)
    };
    let down = |name: &str, x: &BigRational, value: BigRational| {
        Witness::new(
            name.to_string(),
            at(x),
            Kind::Deflection,
            Sense::Down,
            value,
        )
    };
    let l2 = &l * &l;
    let l3 = &l2 * &l;
    match example.id() {
        "E1" => vec![
            moment("wL²/8", &mid, Sense::Sagging, &w * &l2 / n(8)),
            down("5wL⁴/384EI", &mid, n(5) * &w * &l3 * &l / (n(384) * &ei)),
        ],
        "E2" => vec![
            moment("PL/4", &mid, Sense::Sagging, &p * &l / n(4)),
            down("PL³/48EI", &mid, &p * &l3 / (n(48) * &ei)),
        ],
        "E3" => vec![
            moment("Pab/L", &a, Sense::Sagging, &p * &a * &b / &l),
            down("Pa²b²/3EIL", &a, &p * &a * &a * &b * &b / (n(3) * &ei * &l)),
        ],
        "E4" => vec![
            moment("PL", &n(0), Sense::Hogging, &p * &l),
            down("PL³/3EI", &l, &p * &l3 / (n(3) * &ei)),
        ],
        "E5" => {
            let sections: Vec<BigRational> = example
                .moment_sections()
                .iter()
                .map(|x| x.value().clone())
                .collect();
            let name = |x: &BigRational| {
                format!(
                    "E1 + E3 at {} ft",
                    fraction(scale(&at(x), &frac!(1, 12)).value())
                )
            };
            let uniform_m = |x: &BigRational| &w * x * (&l - x) / n(2);
            let point_m = |x: &BigRational| {
                if x <= &a {
                    &p * &b * x / &l
                } else {
                    &p * &a * (&l - x) / &l
                }
            };
            let uniform_v =
                |x: &BigRational| &w * x * (&l3 - n(2) * &l * x * x + x * x * x) / (n(24) * &ei);
            let point_v = |x: &BigRational| {
                if x <= &a {
                    &p * &b * x * (&l2 - &b * &b - x * x) / (n(6) * &ei * &l)
                } else {
                    &p * &a * (&l - x) * (n(2) * &l * x - &a * &a - x * x) / (n(6) * &ei * &l)
                }
            };
            let mut all: Vec<Witness> = sections
                .iter()
                .map(|x| moment(&name(x), x, Sense::Sagging, uniform_m(x) + point_m(x)))
                .collect();
            all.extend(
                example
                    .deflection_sections()
                    .iter()
                    .map(|x| x.value().clone())
                    .map(|x| down(&name(&x), &x, uniform_v(&x) + point_v(&x))),
            );
            all
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::stated;
    use crate::beam::examples;
    use crate::bridge::Edition;
    use crate::q::frac;

    #[test]
    fn the_witnesses_state_the_plan_values() {
        let edition = Edition::aisc_for_tests();
        let got: Vec<(String, num_rational::BigRational)> = examples()
            .iter()
            .flat_map(|e| stated(e, &edition))
            .map(|w| (w.name().to_string(), w.value().clone()))
            .collect();
        let want = [
            ("wL²/8", frac!(5184, 5)),
            ("5wL⁴/384EI", frac!(93312, 61625)),
            ("PL/4", frac!(720, 1)),
            ("PL³/48EI", frac!(10368, 12325)),
            ("Pab/L", frac!(540, 1)),
            ("Pa²b²/3EIL", frac!(5832, 12325)),
            ("PL", frac!(600, 1)),
            ("PL³/3EI", frac!(240, 493)),
            ("E1 + E3 at 6 ft", frac!(6588, 5)),
            ("E1 + E3 at 12 ft", frac!(6984, 5)),
            ("E1 + E3 at 6 ft", frac!(478224, 308125)),
            ("E1 + E3 at 12 ft", frac!(128952, 61625)),
        ];
        let want: Vec<(String, num_rational::BigRational)> =
            want.into_iter().map(|(n, v)| (n.to_string(), v)).collect();
        assert_eq!(got, want);
    }
}
