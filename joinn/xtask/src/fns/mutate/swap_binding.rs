//! Replace a body binding's declared hash.

use joinn_frame::{Hash, Verdict};
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn swap_binding(u: &mut Universe, alias: &str, to_hash: &str) -> Verdict<()> {
    let Some(hash) = Hash::parse_hex(to_hash) else {
        return Verdict::Refused(refuse(format!(
            "hash {to_hash} is not a body hash; acceptance is 64 hex digits"
        )));
    };
    let Some(binding) = u.coding.bodies.iter_mut().find(|b| b.alias == alias) else {
        return Verdict::Refused(refuse(format!(
            "no such alias {alias}; acceptance is a declared body alias"
        )));
    };
    binding.hash = hash;
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::load_phase5_bodies::load_phase5_bodies;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::workspace_root::workspace_root;
    use joinn_dna::{hash, parse_body};
    use joinn_frame::{FrameRegistry, Verdict};
    use joinn_link::{bind_bodies, check_link_types, hash_universe};
    use std::fs;

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src)
            .unwrap_or_else(|e| panic!("parse universe: {e}"))
    }

    fn echo_hash_hex() -> String {
        let root = workspace_root().unwrap_or_else(|e| panic!("workspace root: {e}"));
        let src = fs::read_to_string(root.join("corpus/phase5/controls/echo.body"))
            .unwrap_or_else(|e| panic!("read echo.body: {e}"));
        let body = match parse_body(&src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        hash(&body.coding).to_hex()
    }

    #[test]
    fn swap_binding_frame_mismatch_refuses() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let echo = echo_hash_hex();
        let echo_static: &'static str = Box::leak(echo.into_boxed_str());
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::SwapBinding("units", echo_static)) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        let supplied = load_phase5_bodies().unwrap_or_else(|e| panic!("load phase5 bodies: {e}"));
        let bound = match bind_bodies(u, &supplied) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match check_link_types(u, &bound) {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("Text") && r.reason.contains("ℤ"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(()) => panic!("typing must refuse swapped binding frames"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::SwapBinding("ghost", echo_static)) else {
            panic!("missing alias must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
