//! `5` and `3` at the turn of Sum.

use joinn_frame::{Frame, IntFrame, Term, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::add_int_turn0::AddIntTurn0;
use super::refuse::refuse;

/// `5` and `3` at the turn of Sum: the missing port is `2`.
pub fn five_and_three_at_turn() -> Verdict<Value> {
    let five = match IntFrame::new().canonicalize(Term::int(5)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let three = match IntFrame::new().canonicalize(Term::int(3)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    match AddIntTurn0.apply(&BTreeMap::from([(1, three), (2, five)])) {
        Verdict::Ok(m) => match m.get(&0) {
            Some(v) => Verdict::Ok(v.clone()),
            None => Verdict::Refused(refuse("turn produced no port 0")),
        },
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}

#[cfg(test)]
mod tests {
    use super::five_and_three_at_turn;
    use joinn_frame::{Frame, IntFrame, Term, Verdict};

    #[test]
    fn five_three_is_two() {
        match five_and_three_at_turn() {
            Verdict::Ok(v) => {
                let two = match IntFrame::new().canonicalize(Term::int(2)) {
                    Verdict::Ok(t) => t,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                assert_eq!(v, two);
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
