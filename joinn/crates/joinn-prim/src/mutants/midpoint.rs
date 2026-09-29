//! Mutant `midpoint`: ⌊(a + b) / 2⌋. Commutative, not associative.

use crate::alleles::{canon_int, int_of};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use num_bigint::BigInt;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::two_in::two_in;

pub struct Midpoint;
impl Oracle for Midpoint {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        two_in(inputs).and_then(|(a, b)| match (int_of(&a), int_of(&b)) {
            (Some(x), Some(y)) => {
                let total = x + y;
                let two = BigInt::from(2);
                let truncated = &total / &two;
                let floor = if &truncated * &two > total {
                    truncated - 1
                } else {
                    truncated
                };
                canon_int(floor).map(|v| BTreeMap::from([(2, v)]))
            }
            _ => Verdict::Refused(refuse("mutant.midpoint")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Midpoint;
    use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};
    use joinn_gate::Oracle;
    use std::collections::BTreeMap;

    fn int(n: i64) -> Value {
        match IntFrame::new().canonicalize(Term::int(n)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn mid(a: i64, b: i64) -> Value {
        match Midpoint.apply(&BTreeMap::from([(0, int(a)), (1, int(b))])) {
            Verdict::Ok(out) => match out.get(&2) {
                Some(v) => v.clone(),
                None => panic!("no out-port 2"),
            },
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn midpoint_floors_toward_negative_infinity() {
        assert_eq!(mid(0, 4), int(2));
        assert_eq!(mid(0, 1), int(0));
        assert_eq!(mid(-1, 0), int(-1));
        assert_eq!(mid(-3, 0), int(-2));
    }

    #[test]
    fn the_plan_counterexample_breaks_associativity() {
        assert_eq!(mid(0, 0), int(0));
        assert_eq!(mid(0, 4), int(2));
        assert_eq!(mid(0, 2), int(1));
    }
}
