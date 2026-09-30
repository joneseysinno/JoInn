//! Pairing doesn't matter: every member → in-port order of every force, run
//! under the script, leaves every response's out-port holding the same value.

use joinn_dna::{Cell, Contact, Direction};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_test_host::run;
use std::collections::BTreeMap;

use super::lower_paired::lower_paired;
use super::permutations::permutations;
use super::script_events::script_events;

/// `pairings: <n>, <response>@<port> = <value> in each`. The first order is the
/// canonical one; any order that disagrees with it is the failure.
pub(super) fn pairings(
    contact: &Contact,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
) -> Result<String, String> {
    let mut combos: Vec<BTreeMap<String, Vec<u32>>> = vec![BTreeMap::new()];
    for force in &contact.coding.forces {
        let k = u32::try_from(force.members.len()).map_err(|e| e.to_string())?;
        let mut next = Vec::new();
        for combo in &combos {
            for order in permutations(k) {
                let mut c = combo.clone();
                c.insert(force.name.clone(), order);
                next.push(c);
            }
        }
        combos = next;
    }
    let natives = joinn_prim::sealed_natives();
    let mut canonical: Option<Vec<String>> = None;
    for combo in &combos {
        let body = match lower_paired(contact, cells, frames, combo) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => return Err(format!("pairing {combo:?}: refused: {}", r.reason)),
        };
        let cap = match run(body, cells.clone(), natives.clone(), script_events()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(format!("pairing {combo:?}: refused: {}", r.reason)),
        };
        let mut values = Vec::new();
        for force in &contact.coding.forces {
            let Some(d) = cap
                .descriptions
                .iter()
                .rev()
                .find(|d| d.instance == force.name)
            else {
                return Err(format!("pairing {combo:?}: {} never fired", force.name));
            };
            for port in d.ports.iter().filter(|p| p.direction == Direction::Out) {
                values.push(format!(
                    "{}@{} = {}",
                    force.name,
                    port.position,
                    port.value.as_deref().unwrap_or("(empty)")
                ));
            }
        }
        match &canonical {
            None => canonical = Some(values),
            Some(want) if *want == values => {}
            Some(want) => {
                return Err(format!(
                    "pairing {combo:?} gives {}; the canonical order gives {}",
                    values.join(", "),
                    want.join(", ")
                ));
            }
        }
    }
    Ok(format!(
        "pairings: {}, {} in each",
        combos.len(),
        canonical.unwrap_or_default().join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::pairings;
    use crate::fns::contact::lower_paired::lower_paired;
    use crate::fns::contact::script_events::script_events;
    use crate::fns::forces::corpus_cells;
    use joinn_dna::parse_contact;
    use joinn_frame::{FrameRegistry, Verdict};
    use std::collections::BTreeMap;

    #[test]
    fn every_pairing_keeps_the_sum_and_only_the_presentation_moves() {
        let frames = FrameRegistry::phase1();
        let cells = corpus_cells(&frames).unwrap_or_else(|e| panic!("{e}"));
        let contact = match parse_contact(
            include_str!("../../../../corpus/phase7/calculator.contact"),
            &frames,
        ) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(
            pairings(&contact, &cells, &frames),
            Ok("pairings: 2, sum@2 = 5 in each".to_owned())
        );
        let mut inputs = Vec::new();
        for order in [vec![0, 1], vec![1, 0]] {
            let orders = BTreeMap::from([("sum".to_owned(), order)]);
            let body = match lower_paired(&contact, &cells, &frames, &orders) {
                Verdict::Ok(b) => b,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let cap = match joinn_test_host::run(
                body,
                cells.clone(),
                joinn_prim::sealed_natives(),
                script_events(),
            ) {
                Verdict::Ok(c) => c,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let Some(sum) = cap.descriptions.iter().rev().find(|d| d.instance == "sum") else {
                panic!("sum never fired");
            };
            let values: Vec<&str> = sum
                .ports
                .iter()
                .filter_map(|p| p.value.as_deref())
                .collect();
            inputs.push(values.join(" "));
        }
        assert_eq!(inputs, ["2 3 5", "3 2 5"]);
    }
}
