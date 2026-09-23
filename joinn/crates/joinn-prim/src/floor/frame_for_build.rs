//! `build`'s frame selection from a witness.

use joinn_frame::{Frame, FrameId, RatFrame, TextFrame, Value};

use super::frame_of::frame_of;
use super::int_tag::int_tag;

/// `build`'s frame. A pair belongs to no frame (R38); an ℤ-tagged pair
/// whose left part is `1` selects Text, `2` selects ℚ. That is how a
/// reference with only a ℤ input constructs a Text value without `hash`.
pub(in crate::floor) fn frame_for_build(witness: &Value) -> Box<dyn Frame> {
    if witness.frame().id == FrameId::Int {
        if let Some((kind, _)) = witness.unproduct() {
            if let Some(n) = int_tag(&kind) {
                match n {
                    1 => return Box::new(TextFrame::new()),
                    2 => return Box::new(RatFrame::new()),
                    _ => {}
                }
            }
        }
    }
    frame_of(witness)
}

#[cfg(test)]
mod tests {
    use super::super::build_prim::BuildPrim;
    use super::super::canon_int::canon_int;
    use super::super::pair::Pair;
    use joinn_frame::{FrameId, Verdict};
    use joinn_gate::Oracle;
    use std::collections::BTreeMap;

    #[test]
    fn build_pair_one_selects_text() {
        let one = match canon_int(1) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let zero = match canon_int(0) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let two = match canon_int(2) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let forty_eight = match canon_int(48) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let sel = match Pair.apply(&BTreeMap::from([(0, one.clone()), (1, zero)])) {
            Verdict::Ok(m) => match m.get(&0).cloned() {
                Some(v) => v,
                None => panic!("pair"),
            },
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match BuildPrim.apply(&BTreeMap::from([(0, sel), (1, two), (2, forty_eight)])) {
            Verdict::Ok(m) => {
                let Some(t) = m.get(&0) else {
                    panic!("chr");
                };
                assert_eq!(t.frame().id, FrameId::Text);
                assert_eq!(t.print_term(), "\"0\"");
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
