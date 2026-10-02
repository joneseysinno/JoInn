use num_traits::Zero;

use super::Station;
use crate::q::{Q, frac, scale};
use crate::show::fraction;
use crate::verdict::{Verdict, refuse};

/// M at the far end must be exactly 0: balance seen from the other end.
pub fn closure(id: &str, stations: &[Station]) -> Verdict<Q> {
    let Some(end) = stations.last() else {
        return refuse(format!(
            "balance: {id} has no points; acceptance is a complex with both ends"
        ));
    };
    if !end.m.value().is_zero() {
        return refuse(format!(
            "balance: {id} M at end {} kip·ft, want 0; acceptance is one order for every moment",
            fraction(scale(&end.m, &frac!(1, 12)).value())
        ));
    }
    Verdict::Admitted(end.m.clone())
}
