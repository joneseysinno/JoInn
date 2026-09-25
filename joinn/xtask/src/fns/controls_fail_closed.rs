//! Controls must answer false on a CorruptHash mutant that is not their fact.

#[cfg(test)]
mod tests {
    use crate::fns::g5_assemble_control::g5_assemble_control;
    use crate::fns::g5_law4_control::g5_law4_control;
    use crate::fns::g51_frame_control::g51_frame_control;
    use crate::fns::g51_tails_control::g51_tails_control;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use joinn_frame::Verdict;

    #[test]
    fn corrupt_hash_does_not_flip_unrelated_controls() {
        let src = include_str!("../../../corpus/phase5/universe.universe");
        let subject = parse_subject("phase5/universe.universe", src)
            .unwrap_or_else(|e| panic!("parse universe: {e}"));
        let Verdict::Ok(mutant) = mutate(&subject, &Mutation::CorruptHash("calc")) else {
            panic!("CorruptHash(calc) must apply");
        };
        assert!(
            !g5_assemble_control(&mutant),
            "assemble must not fail open on CorruptHash"
        );
        assert!(
            !g51_tails_control(&mutant),
            "tails must not fail open on CorruptHash"
        );
        assert!(
            !g51_frame_control(&mutant),
            "frame must not fail open on CorruptHash"
        );
        assert!(
            !g5_law4_control(&mutant),
            "law4 must not fail open on CorruptHash"
        );
    }
}
