//! Corrupt the last hex digit of a binding's declared hash.

use joinn_frame::{Hash, Verdict};
use joinn_link::Universe;

use super::refuse::refuse;

pub(super) fn corrupt_hash(u: &mut Universe, alias: &str) -> Verdict<()> {
    let Some(binding) = u.coding.bodies.iter_mut().find(|b| b.alias == alias) else {
        return Verdict::Refused(refuse(format!(
            "no such alias {alias}; acceptance is a declared body alias"
        )));
    };
    let mut hex = binding.hash.to_hex();
    let Some(last) = hex.pop() else {
        return Verdict::Refused(refuse("empty hash"));
    };
    let n = match last {
        '0'..='9' => (last as u8 - b'0') as u32,
        'a'..='f' => (last as u8 - b'a') as u32 + 10,
        'A'..='F' => (last as u8 - b'A') as u32 + 10,
        _ => return Verdict::Refused(refuse(format!("not a hex digit: {last}"))),
    };
    let next = (n + 1) % 16;
    let ch = b"0123456789abcdef"[next as usize] as char;
    hex.push(ch);
    let Some(hash) = Hash::parse_hex(&hex) else {
        return Verdict::Refused(refuse("corrupted hash failed to parse"));
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
    use joinn_frame::Verdict;
    use joinn_link::{bind, hash_universe};

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src)
            .unwrap_or_else(|e| panic!("parse universe: {e}"))
    }

    #[test]
    fn corrupt_hash_binding_refuses_calc() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::CorruptHash("calc")) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        let Some(calc) = u.coding.bodies.iter().find(|b| b.alias == "calc") else {
            panic!("calc binding");
        };
        let declared = calc.hash.to_hex();
        assert_eq!(declared.len(), 64, "{declared}");
        let store = load_phase5_bodies().unwrap_or_else(|e| panic!("load phase5 bodies: {e}"));
        match bind(u, &store) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("calc"), "{}", r.reason);
                assert!(r.reason.contains(&declared), "{}", r.reason);
            }
            Verdict::Ok(_) => panic!("binding must refuse corrupted hash"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::CorruptHash("ghost")) else {
            panic!("missing alias must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
