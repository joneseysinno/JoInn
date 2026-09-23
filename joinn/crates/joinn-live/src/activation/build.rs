//! Build an activation from a body and cell map.

use joinn_dna::{Body, Cell, GenomeTarget};
use joinn_frame::{CheckId, Hash, Refusal, Subject, Verdict};
use joinn_prim::floor;
use std::collections::{BTreeMap, BTreeSet};

use crate::activation::{Activation, Instance};
use crate::slot::Slot;

pub(crate) fn build_activation(
    body: &Body,
    cells: &BTreeMap<Hash, Cell>,
    return_to: Option<String>,
    label: &str,
    nested: bool,
    seed: u64,
) -> Verdict<Activation> {
    let mut instances = BTreeMap::new();
    for g in &body.coding.genome {
        match &g.target {
            GenomeTarget::Prim(name) => {
                if !floor::contains(name) {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Contract,
                        subject: Subject::Other(name.clone()),
                        reason: format!("prim:{name} is not in the floor"),
                        counterexample: None,
                        seed,
                    });
                }
                let Some(cell) = floor::prim_cell(name) else {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Contract,
                        format!("prim:{name} has no contract"),
                    ));
                };
                for inst_name in &g.instances {
                    let mut slots = BTreeMap::new();
                    for p in &cell.coding.contract.ports {
                        slots.insert(p.position, Slot::Empty);
                    }
                    instances.insert(
                        inst_name.clone(),
                        Instance {
                            cell: cell.clone(),
                            slots,
                            is_prim: true,
                        },
                    );
                }
            }
            GenomeTarget::Cell(h) => {
                let Some(cell) = cells.get(h) else {
                    return Verdict::Refused(Refusal {
                        check: CheckId::Contract,
                        subject: Subject::Cell(*h),
                        reason: format!("unknown cell hash {}", h.to_hex()),
                        counterexample: None,
                        seed,
                    });
                };
                for inst_name in &g.instances {
                    let mut slots = BTreeMap::new();
                    for p in &cell.coding.contract.ports {
                        slots.insert(p.position, Slot::Empty);
                    }
                    instances.insert(
                        inst_name.clone(),
                        Instance {
                            cell: cell.clone(),
                            slots,
                            is_prim: false,
                        },
                    );
                }
            }
        }
    }
    let mut grant_holder = BTreeMap::new();
    if !nested {
        for (cap, insts) in &body.coding.grants {
            if let Some(first) = insts.first() {
                grant_holder.insert(cap.clone(), first.clone());
            }
        }
    }
    let boundary = if nested {
        if instances.contains_key("self") {
            Some("self".into())
        } else {
            instances.keys().next().cloned()
        }
    } else {
        None
    };
    Verdict::Ok(Activation {
        body: body.clone(),
        instances,
        queue: BTreeSet::new(),
        mail: BTreeMap::new(),
        return_to,
        boundary,
        label: label.into(),
        grant_holder,
        nested,
    })
}

#[cfg(test)]
mod tests {
    use crate::state::BodyState;
    use crate::state::fixtures::natives;
    use joinn_dna::parse_body;
    use joinn_frame::{FrameRegistry, Verdict};
    use std::collections::BTreeMap;

    #[test]
    fn prim_nonesuch_is_refused_by_name() {
        let src = "body { codex 1 genome { prim:nonesuch as x } grants { } wires { } budget { steps 1 } lineage none }\n";
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match BodyState::new(body, BTreeMap::new(), natives(), 1) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("nonesuch"), "{}", r.reason);
            }
            Verdict::Ok(_) => panic!("prim:nonesuch must refuse"),
        }
    }

    #[test]
    fn prim_succ_is_refused_by_name() {
        let src = "body { codex 1 genome { prim:succ as s } grants { } wires { } budget { steps 1 } lineage none }\n";
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match BodyState::new(body, BTreeMap::new(), natives(), 1) {
            Verdict::Refused(r) => assert!(r.reason.contains("succ"), "{}", r.reason),
            Verdict::Ok(_) => panic!("prim:succ must refuse"),
        }
    }
}
