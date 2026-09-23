//! Lookup. Partial: refuses when the store does not hold the hash.

use joinn_frame::{Term, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::content_store::content_store;
use super::refuse::refuse;
use super::{Opposition, Reference, Register};

/// Lookup. Partial: refuses when the store does not hold the hash.
pub struct ResolvePrim;
impl Oracle for ResolvePrim {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("resolve missing hash"));
        };
        let Term::Text(hex) = v.term() else {
            return Verdict::Refused(refuse("resolve expects a Text hash"));
        };
        match content_store().lock() {
            Ok(g) => match g.get(hex) {
                Some(found) => Verdict::Ok(BTreeMap::from([(0, found.clone())])),
                None => Verdict::Refused(refuse(&format!("resolve: absent hash {hex}"))),
            },
            Err(_) => Verdict::Refused(refuse("content store poisoned")),
        }
    }
}
impl Reference for ResolvePrim {
    fn name(&self) -> &'static str {
        "resolve"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("hash")
    }
    fn register(&self) -> Register {
        Register::Physics
    }
}

#[cfg(test)]
mod tests {
    use super::ResolvePrim;
    use joinn_frame::{Frame, Term, TextFrame, Verdict};
    use joinn_gate::Oracle;
    use std::collections::BTreeMap;

    #[test]
    fn resolve_absent_hash_refuses() {
        let h = match TextFrame::new().canonicalize(Term::text(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        )) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match ResolvePrim.apply(&BTreeMap::from([(0, h)])) {
            Verdict::Refused(r) => assert!(r.reason.contains("absent"), "{}", r.reason),
            Verdict::Ok(_) => panic!("absent hash must refuse"),
        }
    }
}
