//! Run sealed vs reference vs counterfeit.

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use std::collections::BTreeMap;

use super::drive_admits::drive_admits;
use super::drive_frame::drive_frame;
use super::out_port::out_port;
use super::refuse_seal::refuse_seal;
use super::register_seal::register_seal;
use super::round_trip::round_trip;
use super::sample_inputs::sample_inputs;
use super::sealed_oracle::sealed_oracle;
use super::truth_violation::truth_violation;
use super::{DnaFire, Seal};

/// Run sealed vs reference vs counterfeit. Ports come from the cell's contract.
pub fn agree(
    dna: &dyn DnaFire,
    bodies: &BTreeMap<Hash, Body>,
    cells: &BTreeMap<Hash, Cell>,
    seals: &[Seal],
    seed: u64,
    n: u32,
) -> Verdict<Vec<String>> {
    let mut lines = Vec::new();
    for seal in seals {
        match register_seal(seal) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
        let Some(cell) = cells.get(&seal.cell) else {
            return Verdict::Refused(refuse_seal(
                &seal.sealed.0,
                &format!("agree: no cell loaded for {}", seal.sealed.0),
            ));
        };
        let Some(body) = bodies.get(&seal.reference.hash) else {
            return Verdict::Refused(refuse_seal(
                &seal.sealed.0,
                &format!("agree: no reference body loaded for {}", seal.sealed.0),
            ));
        };
        let Some(cf_body) = bodies.get(&seal.counterfeit.hash) else {
            return Verdict::Refused(refuse_seal(
                &seal.sealed.0,
                &format!("agree: no counterfeit body loaded for {}", seal.sealed.0),
            ));
        };
        let Some(sealed) = sealed_oracle(&seal.sealed.0) else {
            return Verdict::Refused(refuse_seal(
                &seal.sealed.0,
                &format!("agree: no sealed oracle for {}", seal.sealed.0),
            ));
        };
        let out = out_port(cell);
        let admits = drive_admits(drive_frame(cell, seal.drives.port), seal.drives.bound);
        let mut separated: Option<(u32, String)> = None;
        for i in 0..n {
            let inputs = sample_inputs(cell, &seal.drives, seed, i);
            let se = match sealed.apply(&inputs) {
                Verdict::Ok(m) => Some(m),
                Verdict::Refused(r) => match dna.fire(body, cell, cells, bodies, &inputs) {
                    Verdict::Refused(_) => None,
                    Verdict::Ok(_) => {
                        return Verdict::Refused(truth_violation(
                            &seal.sealed.0,
                            seed.wrapping_add(u64::from(i)),
                            &inputs,
                            None,
                            None,
                            &format!("sealed refused ({}); reference accepted", r.reason),
                        ));
                    }
                },
            };
            let Some(se) = se else {
                let cf = dna.fire(cf_body, cell, cells, bodies, &inputs);
                if matches!(cf, Verdict::Ok(_)) && separated.is_none() {
                    separated = Some((i, "sealed refused; counterfeit accepted".into()));
                }
                continue;
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
            match dna.fire(cf_body, cell, cells, bodies, &inputs) {
                Verdict::Ok(cf) if cf != se => {
                    if separated.is_none() {
                        let sample = se
                            .get(&out)
                            .or_else(|| inputs.values().next())
                            .map(|v| v.print_term())
                            .unwrap_or_default();
                        separated = Some((i, sample));
                    }
                }
                Verdict::Refused(_) => {
                    if separated.is_none() {
                        separated = Some((i, "counterfeit refused".into()));
                    }
                }
                Verdict::Ok(_) => {}
            }
        }
        match separated {
            Some((i, sample)) => {
                lines.push(format!(
                    "{}: reference ≡ sealed on {n} samples (seed {seed}); counterfeit separated at sample {i} ({sample}); drive port {} bound {} admits {admits} values",
                    seal.sealed.0,
                    seal.drives.port,
                    seal.drives.bound
                ));
            }
            None => {
                return Verdict::Refused(refuse_seal(
                    &seal.sealed.0,
                    &format!(
                        "BLIND SEAL {}: counterfeit body {} agreed on all {n} samples\n  drive port {} bound {} admits {admits} values; widen it or the harness sees nothing",
                        seal.sealed.0,
                        seal.counterfeit.hash.to_hex(),
                        seal.drives.port,
                        seal.drives.bound
                    ),
                ));
            }
        }
    }
    match round_trip(dna, bodies, cells, seals, seed) {
        Verdict::Ok(extra) => lines.extend(extra),
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    Verdict::Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::super::{AGREE_SAMPLES, ROUNDTRIP_SAMPLES};

    #[test]
    fn agree_samples_constant_is_256() {
        assert_eq!(AGREE_SAMPLES, 256);
        assert_eq!(ROUNDTRIP_SAMPLES, 10_000);
    }
}
