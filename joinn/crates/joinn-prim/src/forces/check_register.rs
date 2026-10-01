//! Admit the force register: every response is a response, order-blind, and opposed.

use joinn_dna::{AlleleBody, Cell, Direction};
use joinn_frame::{CheckId, FrameRegistry, Hash, Refusal, Subject, Verdict};
use joinn_gate::NativeRegistry;
use std::collections::BTreeMap;

use super::{REGISTER_BOUND, REGISTER_SEED, RegisterRow, order_blind};

/// For every row, in order:
/// 1. the response cell is supplied, has in-ports `0 … k−1` and one out-port
///    `k`, all in the row's frame, and an allele for that frame whose native
///    is registered;
/// 2. every such native passes `order_blind` at `REGISTER_SEED` and
///    `REGISTER_BOUND`;
/// 3. the separate cell is supplied, its lineage is the response, and it
///    declares a turn.
///
/// The first failure is the refusal.
pub fn check_register(
    rows: &[RegisterRow],
    cells: &BTreeMap<Hash, Cell>,
    natives: &NativeRegistry,
    frames: &FrameRegistry,
) -> Verdict<()> {
    let refuse = |subject: Hash, reason: String| {
        Verdict::Refused(Refusal {
            check: CheckId::Laws,
            subject: Subject::Cell(subject),
            reason,
            counterexample: None,
            seed: REGISTER_SEED,
        })
    };
    for row in rows {
        let word = row.force.word();
        let Some(frame_ref) = row.frame_ref() else {
            return refuse(
                row.response,
                format!(
                    "{word} on {} {}: no such frame; acceptance is a frame JoInn knows (Text, ℤ, ℚ)",
                    row.frame.0, row.frame.1
                ),
            );
        };
        let Some(frame) = frames.get(&frame_ref) else {
            return refuse(
                row.response,
                format!(
                    "{word} on {frame_ref}: the frame is not registered; acceptance is a frame in the frame registry"
                ),
            );
        };
        let Some(cell) = cells.get(&row.response) else {
            return refuse(
                row.response,
                format!(
                    "{word} on {frame_ref}: response cell:{} was not supplied; acceptance is that cell in the cell map",
                    row.response
                ),
            );
        };
        let ports = &cell.coding.contract.ports;
        let ins: Vec<_> = ports
            .iter()
            .filter(|p| p.direction == Direction::In)
            .collect();
        let k = ins.len();
        let shaped = k >= 1
            && ports.len() == k + 1
            && ins
                .iter()
                .enumerate()
                .all(|(i, p)| p.position as usize == i && p.frame == frame_ref)
            && ports.iter().any(|p| {
                p.direction == Direction::Out && p.position as usize == k && p.frame == frame_ref
            });
        if !shaped {
            return refuse(
                row.response,
                format!(
                    "{word} on {frame_ref}: cell:{} is not a response; acceptance is in-ports 0 … k−1 and one out-port k, all in {frame_ref}",
                    row.response
                ),
            );
        }
        let oracles: Vec<_> = cell
            .alleles
            .iter()
            .filter_map(|allele| match &allele.body {
                AlleleBody::Native(id) if allele.frame == frame_ref => natives.get(id),
                _ => None,
            })
            .collect();
        if oracles.is_empty() {
            return refuse(
                row.response,
                format!(
                    "{word} on {frame_ref}: cell:{} has no allele for {frame_ref} whose native is registered; acceptance is a native allele in {frame_ref}",
                    row.response
                ),
            );
        }
        for oracle in oracles {
            if let Verdict::Refused(r) = order_blind(oracle, frame, REGISTER_SEED, REGISTER_BOUND) {
                return refuse(
                    row.response,
                    format!(
                        "{word} on {frame_ref} by cell:{}: {}",
                        row.response, r.reason
                    ),
                );
            }
        }
        let opposed = cells.get(&row.separate).is_some_and(|separate| {
            separate.coding.lineage == Some(row.response) && !separate.coding.turns.is_empty()
        });
        if !opposed {
            return refuse(
                row.separate,
                format!(
                    "{word} on {frame_ref} is unopposed: no separate; acceptance is a turn of cell:{}",
                    row.response
                ),
            );
        }
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_register;
    use crate::forces::register;
    use crate::natives_with_mutants; // allow(vocab): tests inject mutants
    use joinn_dna::{Allele, AlleleBody, Cell, NativeId, hash, parse_cell};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use std::collections::BTreeMap;

    fn cells() -> BTreeMap<Hash, Cell> {
        let frames = FrameRegistry::phase1();
        let mut out = BTreeMap::new();
        for src in [
            include_str!("../../../../corpus/phase0/sum.cell"),
            include_str!("../../../../corpus/phase2/sum_turn.cell"),
        ] {
            match parse_cell(src, &frames) {
                Verdict::Ok(c) => {
                    out.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        out
    }

    #[test]
    fn the_register_is_admitted() {
        let got = check_register(
            register(),
            &cells(),
            &natives_with_mutants(), // allow(vocab): tests inject mutants
            &FrameRegistry::phase1(),
        );
        assert!(matches!(got, Verdict::Ok(())), "{got:?}");
    }

    #[test]
    fn a_row_opposed_by_its_own_response_is_unopposed() {
        let mut row = register()[0];
        row.separate = row.response;
        match check_register(
            &[row],
            &cells(),
            &natives_with_mutants(), // allow(vocab): tests inject mutants
            &FrameRegistry::phase1(),
        ) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "combine on ℤ 1 is unopposed: no separate; acceptance is a turn of cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39"
            ),
            Verdict::Ok(()) => panic!("an unopposed row was admitted"),
        }
    }

    #[test]
    fn a_second_allele_that_depends_on_order_is_refused() {
        let row = register()[0];
        let mut with_difference = cells();
        let Some(sum) = with_difference.get_mut(&row.response) else {
            panic!("the sum cell was not supplied");
        };
        let frame = sum.coding.contract.ports[0].frame;
        sum.alleles.push(Allele {
            frame,
            body: AlleleBody::Native(NativeId("mutant.difference".into())),
            witnesses: Vec::new(),
        });
        match check_register(
            &[row],
            &with_difference,
            &natives_with_mutants(), // allow(vocab): tests inject mutants
            &FrameRegistry::phase1(),
        ) {
            Verdict::Refused(r) => assert!(
                r.reason.starts_with(
                    "combine on ℤ 1 by cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39: not order-blind: f(a, b) = "
                ) && r.reason.contains(" but f(b, a) = "),
                "{}",
                r.reason
            ),
            Verdict::Ok(()) => panic!("a second, order-dependent allele passed the register"),
        }
    }

    #[test]
    fn a_missing_response_cell_is_refused_by_name() {
        let mut row = register()[0];
        row.response = row.separate;
        let mut only_sum = cells();
        only_sum.remove(&register()[0].separate);
        match check_register(
            &[row],
            &only_sum,
            &natives_with_mutants(), // allow(vocab): tests inject mutants
            &FrameRegistry::phase1(),
        ) {
            Verdict::Refused(r) => assert!(r.reason.contains("was not supplied"), "{}", r.reason),
            Verdict::Ok(()) => panic!("a missing response was admitted"),
        }
    }
}
