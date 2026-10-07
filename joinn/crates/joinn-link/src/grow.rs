//! Growth: a system and the inputs its body accepted, exact at every size.

mod accept_word;
mod check_evolution;
mod count_witness;
mod grow_step;
mod identity;
mod lower_grown;
mod respond;
mod transcripts;

pub use accept_word::accept_word;
pub use check_evolution::{Evolution, check_evolution};
pub use count_witness::count_witness;
pub use grow_step::grow_step;
pub use lower_grown::lower_grown;
pub use respond::respond;
pub use transcripts::transcripts;

use joinn_dna::{Cell, Contact, System, SystemForce};
use joinn_frame::{FrameRegistry, Hash, Value, Verdict};
use std::collections::BTreeMap;

use crate::check_system::check_system;

/// A system and the inputs its body accepted, in growth order. It has no hash
/// of its own and is never printed into a coding region or written.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grown {
    system: System,
    contact: Contact,
    inputs: Vec<Value>,
}

impl Grown {
    /// The system that grew.
    pub fn system(&self) -> &System {
        &self.system
    }

    /// The growing body's contact.
    pub fn contact(&self) -> &Contact {
        &self.contact
    }

    /// The accepted inputs, in growth order.
    pub fn inputs(&self) -> &[Value] {
        &self.inputs
    }

    /// The force that reaches the body.
    pub fn force(&self) -> Option<&SystemForce> {
        self.system.coding.forces.first()
    }

    /// The name of grown instance `n`: `numbers.<n>`.
    pub fn instance(&self, n: usize) -> String {
        let name = self.contact.coding.grows.as_ref().map(|g| g.name.as_str());
        format!("{}.{n}", name.unwrap_or("grown"))
    }
}

/// Admit `system` (all but its lineage, which growth never reads), then accept
/// `inputs` one by one. The first input the body does not accept refuses.
pub fn grow(
    system: &System,
    contacts: &BTreeMap<Hash, Contact>,
    cells: &BTreeMap<Hash, Cell>,
    inputs: &[Value],
) -> Verdict<Grown> {
    let mut unparented = system.clone();
    unparented.coding.lineage = None;
    let frames = FrameRegistry::phase1();
    if let Verdict::Refused(r) =
        check_system(&unparented, contacts, &BTreeMap::new(), cells, &frames)
    {
        return Verdict::Refused(r);
    }
    let contact = system
        .coding
        .bodies
        .first()
        .and_then(|b| contacts.get(&b.contact));
    let Some(contact) = contact else {
        return crate::refuse("grow: the system binds no body; acceptance is one growing body");
    };
    let mut grown = Grown {
        system: system.clone(),
        contact: contact.clone(),
        inputs: Vec::new(),
    };
    for input in inputs {
        grown = match grow_step(&grown, input) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
    }
    Verdict::Ok(grown)
}

#[cfg(test)]
pub(crate) mod fixtures;

#[cfg(test)]
mod tests {
    use joinn_dna::hash;
    use joinn_frame::{FrameRegistry, Verdict};

    use super::fixtures::{adding, cells, counting, descendant, int, run};
    use super::{check_evolution, count_witness, grow, grow_step, lower_grown, respond};

    #[test]
    fn count_witness_and_the_engine_agree_on_every_transcript_at_every_step() {
        let cells = cells();
        let frames = FrameRegistry::phase1();
        let (counting, counting_contacts) = counting();
        let (adding, adding_contacts) = adding();
        let twelve: Vec<i64> = (1..=12).collect();
        let cases: Vec<(&str, Vec<i64>, Vec<i64>, bool)> = vec![
            ("counting", vec![], vec![], false),
            ("counting", vec![1, 1, 1], vec![1, 2, 3], false),
            ("counting", vec![1, 1, 3], vec![1, 2], true),
            ("counting", vec![1; 12], twelve.clone(), false),
            ("adding", vec![], vec![], false),
            ("adding", vec![2, 3, 4], vec![2, 5, 9], false),
            ("adding", vec![5, -2, 0, 7], vec![5, 3, 3, 10], false),
            ("adding", vec![1, 1, 1], vec![1, 2, 3], false),
            ("adding", vec![1, 1, 3], vec![1, 2, 5], false),
            ("adding", vec![1; 12], twelve, false),
        ];
        for (name, transcript, counts, refuses_last) in cases {
            let (system, contacts) = if name == "counting" {
                (&counting, &counting_contacts)
            } else {
                (&adding, &adding_contacts)
            };
            let mut grown = match grow(system, contacts, &cells, &[]) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let mut seen = vec![0i64];
            seen.extend(counts.iter().copied());
            for (step, v) in transcript.iter().enumerate() {
                match grow_step(&grown, &int(*v)) {
                    Verdict::Ok(g) => grown = g,
                    Verdict::Refused(r) => {
                        assert!(refuses_last && step + 1 == transcript.len(), "{name} {v}");
                        assert_eq!(
                            r.reason,
                            format!(
                                "counting: {v} is not one; acceptance is 1 (counting grows by one)"
                            )
                        );
                        break;
                    }
                }
            }
            assert_eq!(
                grown.inputs().len(),
                counts.len(),
                "{name} {transcript:?} cells"
            );
            for (n, want) in seen.iter().enumerate() {
                let prefix: Vec<_> = transcript[..n].iter().map(|v| int(*v)).collect();
                let at = match grow(system, contacts, &cells, &prefix) {
                    Verdict::Ok(g) => g,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                let engine = match respond(&at, &cells, &frames) {
                    Verdict::Ok(v) => v,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                let witness = match count_witness(at.inputs()) {
                    Verdict::Ok(v) => v,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                assert_eq!(engine, witness, "{name} {:?}", &transcript[..n]);
                assert_eq!(engine, int(*want), "{name} {:?}", &transcript[..n]);
            }
        }
    }

    #[test]
    fn count_witness_refuses_too_many_steps() {
        match count_witness(&[int(2), int(-1_000_001)]) {
            Verdict::Refused(r) => {
                assert_eq!(
                    r.reason,
                    "witness: too many steps; acceptance is |v| ≤ 1000000"
                );
            }
            Verdict::Ok(v) => panic!("counted {}", v.print_term()),
        }
    }

    #[test]
    fn adding_evolves_counting_and_a_counting_with_nothing_new_is_refused() {
        let cells = cells();
        let (counting, counting_contacts) = counting();
        let (adding, mut contacts) = descendant("any", Some((&counting, &counting_contacts)));
        contacts.extend(counting_contacts.clone());
        match check_evolution(&counting, &adding, &contacts, &cells) {
            Verdict::Ok(e) => {
                assert_eq!(e.witnesses, 3);
                assert_eq!(e.gained, int(3));
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        let (again, mut contacts) = descendant("one", Some((&counting, &counting_contacts)));
        contacts.extend(counting_contacts);
        match check_evolution(&counting, &again, &contacts, &cells) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "evolution: counting accepts nothing counting refuses; acceptance is a new ability (otherwise it is an edit)"
            ),
            Verdict::Ok(e) => panic!("an edit evolved: {e:?}"),
        }
    }

    #[test]
    fn every_size_is_true_on_counting_and_adding() {
        let (counting, counting_contacts) = counting();
        let (adding, adding_contacts) = adding();
        for (n, want) in [(0usize, 0i64), (1, 1), (2, 2), (3, 3), (12, 12)] {
            let ones = vec![int(1); n];
            assert_eq!(
                run(&counting, &counting_contacts, &ones),
                int(want),
                "counting {n}"
            );
        }
        let values = [5i64, -3, 0, 7, 12, -40, 2, 9, 1, 1, 100, -8];
        for n in [0usize, 1, 2, 3, 12] {
            let inputs: Vec<_> = values[..n].iter().map(|v| int(*v)).collect();
            let want: i64 = values[..n].iter().sum();
            assert_eq!(
                run(&adding, &adding_contacts, &inputs),
                int(want),
                "adding {n}"
            );
        }
        assert_eq!(run(&adding, &adding_contacts, &[int(-7)]), int(-7));
    }

    #[test]
    fn counting_refuses_three_in_its_words() {
        let (counting, contacts) = counting();
        match grow(&counting, &contacts, &cells(), &[int(1), int(3)]) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "counting: 3 is not one; acceptance is 1 (counting grows by one)"
            ),
            Verdict::Ok(_) => panic!("counting accepted 3"),
        }
    }

    #[test]
    fn growth_does_not_move_the_system_s_hash() {
        let (adding, contacts) = adding();
        let before = hash(&adding.coding);
        let grown = match grow(&adding, &contacts, &cells(), &[int(4), int(5), int(6)]) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(hash(&grown.system().coding), before);
        assert_eq!(hash(&adding.coding), before);
    }

    #[test]
    fn deltas_equal_regrow_at_every_size() {
        let (adding, contacts) = adding();
        let cells = cells();
        let frames = FrameRegistry::phase1();
        let values = [5i64, -3, 0, 7, 12, -40, 2, 9, 1, 1, 100, -8];
        let mut delta = match grow(&adding, &contacts, &cells, &[]) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for n in 0..=values.len() {
            if n > 0 {
                delta = match grow_step(&delta, &int(values[n - 1])) {
                    Verdict::Ok(g) => g,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
            }
            let inputs: Vec<_> = values[..n].iter().map(|v| int(*v)).collect();
            let regrown = match grow(&adding, &contacts, &cells, &inputs) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(format!("{delta:?}"), format!("{regrown:?}"), "size {n}");
            let lowered = |g| match lower_grown(g, &cells, &frames) {
                Verdict::Ok(b) => joinn_dna::print_body(&b.coding),
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(lowered(&delta), lowered(&regrown), "size {n}");
            let answer = |g| match respond(g, &cells, &frames) {
                Verdict::Ok(v) => v.print_literal(),
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(answer(&delta), answer(&regrown), "size {n}");
        }
    }
}
