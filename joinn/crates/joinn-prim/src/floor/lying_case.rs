//! A CasePrim that reports every Built tag one off the declared order.

use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::canon_int::canon_int;
use super::case_prim::CasePrim;
use super::int_tag::int_tag;

/// A CasePrim that reports every Built tag one off the declared order.
pub struct LyingCase;
impl Oracle for LyingCase {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        match CasePrim.apply(inputs) {
            Verdict::Ok(mut m) => {
                if let Some(tag) = m.get(&0).cloned() {
                    if let Some(n) = int_tag(&tag) {
                        if let Verdict::Ok(shifted) = canon_int(n + 1) {
                            m.insert(0, shifted);
                        }
                    }
                }
                Verdict::Ok(m)
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::case_prim::CasePrim;
    use super::super::int_tag::int_tag;
    use super::LyingCase;
    use joinn_frame::{Frame, IntFrame, Verdict};
    use joinn_gate::Oracle;
    use std::collections::BTreeMap;

    #[test]
    fn lying_case_tag_is_refused_by_value() {
        let int = IntFrame::new();
        let v = int.generate(11, 8);
        let honest = match CasePrim.apply(&BTreeMap::from([(0, v.clone())])) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let lying = match LyingCase.apply(&BTreeMap::from([(0, v)])) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_ne!(
            honest.get(&0),
            lying.get(&0),
            "lying case must shift the tag"
        );
        let Some(tag) = lying.get(&0) else {
            panic!("no tag");
        };
        let n = match int_tag(tag) {
            Some(n) => n,
            None => panic!("tag is ℤ"),
        };
        let honest_tag = match honest.get(&0) {
            Some(t) => t,
            None => panic!("honest tag"),
        };
        let honest_n = match int_tag(honest_tag) {
            Some(n) => n,
            None => panic!("ℤ"),
        };
        assert_eq!(n, honest_n + 1);
    }
}
