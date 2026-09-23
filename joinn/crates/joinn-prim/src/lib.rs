//! The floor, reference bodies, seals, fold/unfold, turn alleles.
//!
//! No scheduling, no IO. Alleles live here; the gate only holds the register.

#![forbid(unsafe_code)]

pub mod alleles;
pub mod floor;
#[cfg(any(test, feature = "mutants"))]
pub mod mutants;
pub mod natives;
pub mod seals;
pub mod turns;

pub use alleles::{add_int_allele, add_rat_allele};
pub use floor::{
    Opposition, Reference, Register, check_oppositions, check_pairing, contains as floor_contains,
    prim_ports, register as floor_register, register_prims,
};
#[cfg(any(test, feature = "mutants"))]
pub use natives::natives_with_mutants; // allow(vocab): cfg-gated mutant register re-export
pub use natives::{engine_natives, sealed_natives};
#[cfg(any(test, feature = "mutants"))]
pub use seals::wrapping_caught_by_agree;
pub use seals::{
    AGREE_SAMPLES, BodyRef, DnaFire, Drive, ROUNDTRIP_SAMPLES, Seal, SealSpec, agree,
    agree_injected_disagreement, agree_one, check_seal_dag, check_v33, check_v33_full,
    find_cell_for_native, fold, mutant_sealed_as_reference_body, register_seal, seal_register,
    unfold, v23_execute, v23_execute_dna,
};
#[cfg(any(test, feature = "mutants"))]
pub use turns::admit_turn_positive_only_is_refused;
pub use turns::{
    AddIntTurn0, AddIntTurn1, admit_add_turn0, admit_turn, five_and_three_at_turn,
    handwritten_turn_alleles, turn_register,
};

#[cfg(test)]
mod tests {
    use super::*;
    use joinn_dna::{Allele, AlleleBody, NativeId, hash, sum_cell};
    use joinn_frame::{FrameRef, Verdict};
    use joinn_gate::{Budget, Gate};

    #[test]
    fn commutativity_breaker_is_refused_with_counter_example() {
        let gate = Gate::new(Budget::default(), natives_with_mutants()); // allow(vocab): tests inject mutants
        let cell = sum_cell();
        let bad = Allele {
            frame: FrameRef::int(),
            body: AlleleBody::Native(NativeId("mutant.difference".into())),
            witnesses: Vec::new(),
        };
        match gate.admit_allele(&cell, &bad) {
            Verdict::Refused(r) => {
                assert!(
                    r.counterexample.is_some(),
                    "refusal must carry a counter-example"
                );
                assert!(
                    r.reason.contains("commutative") || r.check == joinn_frame::CheckId::Laws,
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(_) => panic!("difference allele must be refused"),
        }
    }

    #[test]
    fn failing_founding_witness_names_the_witness() {
        let gate = Gate::new(Budget::default(), natives_with_mutants()); // allow(vocab): tests inject mutants
        let cell = sum_cell();
        let bad = Allele {
            frame: FrameRef::int(),
            body: AlleleBody::Native(NativeId("mutant.zero".into())),
            witnesses: Vec::new(),
        };
        match gate.admit_allele(&cell, &bad) {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("witness")
                        || r.check == joinn_frame::CheckId::Witnesses
                        || r.check == joinn_frame::CheckId::Laws,
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(_) => panic!("constant-zero must be refused"),
        }
    }

    #[test]
    fn cell_hash_does_not_move_when_allele_lives_here() {
        let cell = sum_cell();
        let _ = hash(&cell.coding);
    }

    #[test]
    fn sealed_natives_do_not_contain_mutants() {
        let sealed = sealed_natives();
        assert!(sealed.get(&NativeId("mutant.impostor".into())).is_none());
        assert!(sealed.get(&NativeId("add@ℤ".into())).is_some());
    }

    #[test]
    fn write_slot_is_the_only_path_structural() {
        // V35 lives in joinn-live; this crate must not grow a second fire path.
        assert!(sealed_natives().get(&NativeId("prim:eq".into())).is_some());
        assert!(
            sealed_natives()
                .get(&NativeId("prim:hash".into()))
                .is_none()
        );
    }
}
