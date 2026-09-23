//! Run one seal's sealed oracle against its reference body.

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::out_port::out_port;
use super::refuse::refuse;
use super::register_seal::register_seal;
use super::sample_inputs::sample_inputs;
use super::truth_violation::truth_violation;
use super::{DnaFire, Seal};

/// Run one seal's sealed oracle against its reference body.
pub fn agree_one(
    dna: &dyn DnaFire,
    bodies: &BTreeMap<Hash, Body>,
    cells: &BTreeMap<Hash, Cell>,
    seal: &Seal,
    sealed: &dyn Oracle,
    seed: u64,
    n: u32,
) -> Verdict<()> {
    match register_seal(seal) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let Some(body) = bodies.get(&seal.reference.hash) else {
        return Verdict::Refused(refuse("agree_one: no reference body"));
    };
    let Some(cell) = cells.get(&seal.cell) else {
        return Verdict::Refused(refuse("agree_one: no cell"));
    };
    let out = out_port(cell);
    for i in 0..n {
        let inputs = sample_inputs(cell, &seal.drives, seed, i);
        let se = match sealed.apply(&inputs) {
            Verdict::Ok(m) => m,
            Verdict::Refused(_) => continue,
        };
        let re = match dna.fire(body, cell, cells, bodies, &inputs) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => {
                return Verdict::Refused(truth_violation(
                    &seal.sealed.0,
                    seed.wrapping_add(u64::from(i)),
                    &inputs,
                    se.values().next().cloned(),
                    None,
                    &format!("reference refused ({})", r.reason),
                ));
            }
        };
        if se != re {
            return Verdict::Refused(truth_violation(
                &seal.sealed.0,
                seed.wrapping_add(u64::from(i)),
                &inputs,
                se.get(&out)
                    .cloned()
                    .or_else(|| se.values().next().cloned()),
                re.get(&out)
                    .cloned()
                    .or_else(|| re.values().next().cloned()),
                "reference and sealed alleles disagree",
            ));
        }
    }
    Verdict::Ok(())
}
