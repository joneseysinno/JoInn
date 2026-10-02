use num_rational::BigRational;
use num_traits::{One, Zero};

use super::{Balance, Example, Support};
use crate::q::{Q, add, frac, ratio, scale, total, wedge};
use crate::tag::Tag;
use crate::verdict::{Verdict, admit};

/// ΣF = 0 along y and ΣM = 0 about A, every moment taken as wedge(r, F) with r
/// from A to the force. The uniform load enters as its total over the span,
/// acting at mid-span. Balance takes no bridge: it cannot read one.
pub fn balance(example: &Example) -> Verdict<Balance> {
    let a = Q::new(BigRational::zero(), Tag::PLACE_X);
    let mut load_f = Q::new(BigRational::zero(), Tag::FORCE);
    let mut load_m = Q::new(BigRational::zero(), Tag::MOMENT);
    for p in &example.point_loads {
        load_f = admit!(add(&load_f, &p.force));
        load_m = admit!(add(&load_m, &admit!(wedge(&p.at, &p.force))));
    }
    if let Some(q) = &example.uniform {
        let whole = admit!(total(q, &example.span));
        let mid = scale(&example.span, &frac!(1, 2));
        load_f = admit!(add(&load_f, &whole));
        load_m = admit!(add(&load_m, &admit!(wedge(&mid, &whole))));
    }
    let minus_one = frac!(-1, 1);
    match example.support {
        Support::Simple => {
            let unit = Q::new(BigRational::one(), Tag::FORCE);
            let per_unit = admit!(wedge(&example.span, &unit));
            let r_b = scale(&unit, &(-admit!(ratio(&load_m, &per_unit))));
            let r_a = scale(&admit!(add(&load_f, &r_b)), &minus_one);
            let sum_f = admit!(add(&admit!(add(&load_f, &r_a)), &r_b));
            let reactions_m = admit!(add(
                &admit!(wedge(&a, &r_a)),
                &admit!(wedge(&example.span, &r_b))
            ));
            let sum_m = admit!(add(&load_m, &reactions_m));
            Verdict::Admitted(Balance {
                r_a,
                r_b: Some(r_b),
                couple: None,
                sum_f,
                sum_m,
            })
        }
        Support::Cantilever => {
            let r_a = scale(&load_f, &minus_one);
            let couple = scale(&load_m, &minus_one);
            let sum_f = admit!(add(&load_f, &r_a));
            let sum_m = admit!(add(&load_m, &couple));
            Verdict::Admitted(Balance {
                r_a,
                r_b: None,
                couple: Some(couple),
                sum_f,
                sum_m,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::balance;
    use crate::beam::examples;
    use crate::q::{Q, frac};
    use crate::tag::Tag;
    use crate::verdict::admitted;

    fn kip(q: &Q) -> String {
        assert_eq!(q.tag(), Tag::FORCE);
        crate::show::fraction(q.value())
    }

    #[test]
    fn each_example_balances_with_the_plan_reactions() {
        let got: Vec<(String, String)> = examples()
            .iter()
            .map(|e| {
                let b = admitted(balance(e));
                (kip(b.r_a()), b.r_b().map(kip).unwrap_or_default())
            })
            .collect();
        let want = [
            ("72/5", "72/5"),
            ("5", "5"),
            ("15/2", "5/2"),
            ("5", ""),
            ("219/10", "169/10"),
        ];
        let want: Vec<(String, String)> = want
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        assert_eq!(got, want);
    }

    #[test]
    fn the_cantilever_wall_holds_a_couple_of_600_kip_in() {
        let b = admitted(balance(&examples()[3]));
        assert_eq!(b.couple(), Some(&Q::new(frac!(600, 1), Tag::MOMENT)));
    }

    #[test]
    fn sum_f_and_sum_m_are_zero() {
        for e in examples() {
            let b = admitted(balance(&e));
            assert_eq!(b.sum_f().tag(), Tag::FORCE);
            assert!(b.sum_f().value().is_zero(), "{} ΣF", e.id());
            assert_eq!(b.sum_m().tag(), Tag::MOMENT);
            assert!(b.sum_m().value().is_zero(), "{} ΣM", e.id());
        }
    }
}
