//! `cargo xtask forces`: `check_register` on the register, then the plants of
//! §2.2, all against `natives_with_mutants()`.

use joinn_dna::{AlleleBody, NativeId};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_prim::forces::{
    ForceKind, REGISTER_BOUND, REGISTER_SEED, check_register, order_blind, register,
};
use std::collections::BTreeSet;

use super::corpus_cells;

/// Mutants that must be refused by `order_blind`, with the law each breaks.
const REFUSED: [(&str, &str, &str); 2] = [
    (
        "mutant.difference",
        "difference",
        "not order-blind: f(a, b) = ",
    ),
    (
        "mutant.midpoint",
        "midpoint",
        "not order-blind: f(f(a, b), c) = ",
    ),
];
/// Mutants that are order-blind but wrong: order-blind is necessary, not sufficient.
const UNREGISTERED: [&str; 2] = ["mutant.max", "mutant.plus1"];

/// A refused register, or any plant that answers other than §2.2 says, fails.
pub(crate) fn forces() -> Result<(), String> {
    let frames = FrameRegistry::phase1();
    let cells = corpus_cells(&frames)?;
    let natives = joinn_prim::natives_with_mutants();
    let rows = register();
    if let Verdict::Refused(r) = check_register(rows, &cells, &natives, &frames) {
        return Err(format!("forces: the register is refused: {}", r.reason));
    }
    let mut registered = BTreeSet::new();
    for row in rows {
        let frame_ref = row
            .frame_ref()
            .ok_or_else(|| format!("forces: {} is not a frame", row.frame.0))?;
        let turns = cells
            .get(&row.separate)
            .map(|c| {
                c.coding
                    .turns
                    .iter()
                    .map(|t| {
                        let from: Vec<String> = t.from.iter().map(u32::to_string).collect();
                        format!("turn {} from {{{}}}", t.out, from.join(" "))
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        println!(
            "{} {frame_ref} by cell:{}…: order-blind ({} pairs, {} triples, seed {}), opposed by separate cell:{}… ({turns})",
            row.force.word(),
            row.response.short_hex(),
            REGISTER_BOUND,
            REGISTER_BOUND,
            REGISTER_SEED,
            row.separate.short_hex(),
        );
        if let Some(cell) = cells.get(&row.response) {
            for allele in &cell.alleles {
                if let AlleleBody::Native(id) = &allele.body
                    && allele.frame == frame_ref
                {
                    registered.insert(id.clone());
                }
            }
        }
    }

    let first = rows.first().ok_or("forces: the register is empty")?;
    let frame_ref = first
        .frame_ref()
        .ok_or_else(|| format!("forces: {} is not a frame", first.frame.0))?;
    let frame = frames
        .get(&frame_ref)
        .ok_or_else(|| format!("forces: {frame_ref} is not registered"))?;
    let mut failures: Vec<String> = Vec::new();
    for (native, _, starts) in REFUSED {
        let oracle = natives
            .get(&NativeId(native.into()))
            .ok_or_else(|| format!("forces: {native} is not in natives_with_mutants()"))?;
        match order_blind(oracle, frame, REGISTER_SEED, REGISTER_BOUND) {
            Verdict::Refused(r) if r.reason.starts_with(starts) => {
                println!(
                    "planted: order_blind on {native}: refused (ok): {}",
                    r.reason
                );
            }
            Verdict::Refused(r) => failures.push(format!(
                "planted: order_blind on {native} was refused on the wrong law: {}",
                r.reason
            )),
            Verdict::Ok(()) => failures.push(format!(
                "planted: order_blind on {native} passed; the plant was not refused"
            )),
        }
    }
    for native in UNREGISTERED {
        let id = NativeId(native.into());
        let oracle = natives
            .get(&id)
            .ok_or_else(|| format!("forces: {native} is not in natives_with_mutants()"))?;
        match order_blind(oracle, frame, REGISTER_SEED, REGISTER_BOUND) {
            Verdict::Ok(()) if !registered.contains(&id) => {
                println!("planted: order_blind on {native}: order-blind (ok), not registered");
            }
            Verdict::Ok(()) => failures.push(format!(
                "planted: {native} is a registered response; a wrong answer is in the register"
            )),
            Verdict::Refused(r) => failures.push(format!(
                "planted: order_blind on {native} was refused, but it is order-blind: {}",
                r.reason
            )),
        }
    }
    let mut unopposed = *first;
    unopposed.separate = unopposed.response;
    let want = format!(
        "{} on {frame_ref} is unopposed: no separate; acceptance is a turn of cell:{}",
        first.force.word(),
        first.response
    );
    match check_register(&[unopposed], &cells, &natives, &frames) {
        Verdict::Refused(r) if r.reason == want => {
            println!(
                "planted: check_register with separate = response: refused (ok): {}",
                r.reason
            );
        }
        Verdict::Refused(r) => failures.push(format!(
            "planted: check_register with separate = response was refused for the wrong reason: {}",
            r.reason
        )),
        Verdict::Ok(()) => failures.push(
            "planted: check_register with separate = response was admitted; the plant was not refused"
                .into(),
        ),
    }
    if !failures.is_empty() {
        for f in &failures {
            println!("{f}");
        }
        return Err(format!("forces: {} failure(s)", failures.len()));
    }
    let combines = rows
        .iter()
        .filter(|r| r.force == ForceKind::Combine)
        .count();
    let [(_, first_word, _), (_, second_word, _)] = REFUSED;
    let [max, plus1] = UNREGISTERED.map(|n| n.trim_start_matches("mutant."));
    println!(
        "forces: {combines} combine registered, {} opposed; plants: {first_word} refused, {second_word} refused, {max} and {plus1} order-blind but unregistered, unopposed refused (ok)",
        rows.len()
    );
    Ok(())
}
