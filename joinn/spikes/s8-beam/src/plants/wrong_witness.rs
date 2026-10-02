use num_rational::BigRational;
use num_traits::Signed;

use crate::beam::{Example, Order, derive, station_at};
use crate::bridge::Bridge;
use crate::q::{frac, scale};
use crate::verdict::{Verdict, admit};
use crate::witness::{Kind, Sense, Witness, compare};

/// E1's moment witness replaced by wL²/12. The comparison must refuse it.
pub fn wrong_witness(e1: &Example) -> Verdict<()> {
    let derived = admit!(derive(e1, false, Order::Faithful, &mut Bridge::new(None)));
    let l = e1.span().value();
    let w = e1.uniform().map(|q| q.value().abs()).unwrap_or_default();
    let mid = scale(e1.span(), &frac!(1, 2));
    let witness = Witness::new(
        "wL²/12".to_string(),
        mid.clone(),
        Kind::Moment,
        Sense::Sagging,
        w * l * l / BigRational::from_integer(12.into()),
    );
    let station = admit!(station_at(derived.stations(), &mid));
    compare(e1.id(), &witness, station.m())
}

#[cfg(test)]
mod tests {
    use super::wrong_witness;
    use crate::beam::examples;
    use crate::plants::line;

    #[test]
    fn the_wrong_witness_is_a_truth_violation_with_the_plan_line() {
        let (text, ok) = line("witness wL²/12", wrong_witness(&examples()[0]));
        assert_eq!(
            text,
            "plant witness wL²/12: refused (ok): E1 witness wL²/12 = 288/5 kip·ft, derived 432/5 kip·ft; a disagreement is a truth violation"
        );
        assert!(ok);
    }
}
