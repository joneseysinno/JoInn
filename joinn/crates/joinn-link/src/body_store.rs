//! Bodies keyed only by their computed coding hash.

mod any_cells;
mod get;
mod insert;
mod iter;
mod new;

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use std::collections::BTreeMap;

pub(crate) struct Entry {
    pub(crate) body: Body,
    pub(crate) cells: BTreeMap<Hash, Cell>,
    pub(crate) source: String,
}

/// Bodies keyed by coding hash. The only way to add a body is [`BodyStore::insert`].
#[derive(Default)]
pub struct BodyStore {
    pub(crate) entries: BTreeMap<Hash, Entry>,
}

#[cfg(test)]
mod tests {
    use super::BodyStore;
    use joinn_dna::parse_body;
    use joinn_frame::{FrameRegistry, Verdict};
    use std::collections::BTreeMap;

    #[test]
    fn second_face_is_refused_naming_both() {
        let frames = FrameRegistry::phase1();
        let calc = include_str!("../../../corpus/phase2/calculator.body");
        let variant = include_str!("../../../corpus/phase2/variants/calculator_c.body");
        let body_a = match parse_body(calc, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let body_c = match parse_body(variant, &frames) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut store = BodyStore::new();
        match store.insert(body_a, BTreeMap::new(), "calculator.body") {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        match store.insert(body_c, BTreeMap::new(), "variants/calculator_c.body") {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("calculator.body"), "{}", r.reason);
                assert!(
                    r.reason.contains("variants/calculator_c.body"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(_) => panic!("second face must be refused"),
        }
    }
}
