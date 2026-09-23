//! Content address. Total. The store is the gap to `resolve`.

use joinn_frame::{Frame, Hash, Term, TextFrame, Value, Verdict, keyed_hash};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::content_store::content_store;
use super::refuse::refuse;
use super::{Opposition, Reference, Register};

/// Content address. Total. The store is the gap to `resolve`.
pub struct HashPrim;
impl Oracle for HashPrim {
    fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>> {
        let Some(v) = inputs.get(&0) else {
            return Verdict::Refused(refuse("hash missing argument"));
        };
        let digest: Hash = keyed_hash(b"joinn.hash.v1", v.print_literal().as_bytes());
        let hex = digest.to_hex();
        match content_store().lock() {
            Ok(mut g) => {
                g.insert(hex.clone(), v.clone());
            }
            Err(_) => return Verdict::Refused(refuse("content store poisoned")),
        }
        TextFrame::new()
            .canonicalize(Term::Text(hex))
            .map(|t| BTreeMap::from([(0, t)]))
    }
}
impl Reference for HashPrim {
    fn name(&self) -> &'static str {
        "hash"
    }
    fn opposition(&self) -> Opposition {
        Opposition::Inverse("resolve")
    }
    fn register(&self) -> Register {
        Register::Physics
    }
}

#[cfg(test)]
mod tests {
    use super::super::Register;
    use super::super::{EqPrim, HashPrim, register};

    #[test]
    fn hash_is_physics_and_eq_is_matter() {
        let floor = register();
        let hash = match floor.iter().find(|p| p.name() == "hash") {
            Some(p) => p,
            None => panic!("hash"),
        };
        let eq = match floor.iter().find(|p| p.name() == "eq") {
            Some(p) => p,
            None => panic!("eq"),
        };
        assert_eq!(hash.register(), Register::Physics);
        assert_eq!(eq.register(), Register::Matter);
        let _ = (HashPrim, EqPrim);
    }
}
