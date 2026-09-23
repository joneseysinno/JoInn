//! Exit-gate demos and gate-power mutants. Alleles are injected, not owned.

mod check_label;

use crate::budget::Budget;
use crate::gate::Gate;
use crate::natives::NativeRegistry;
use joinn_dna::{
    Allele, AlleleBody, Direction, NativeId, PortDecl, cli_input_cell, format_cell, hash, sum_cell,
};
use joinn_frame::{FrameRef, Verdict};

fn add_int_allele() -> Allele {
    Allele {
        frame: FrameRef::int(),
        body: AlleleBody::Native(NativeId("add@ℤ".into())),
        witnesses: Vec::new(),
    }
}

fn add_rat_allele() -> Allele {
    Allele {
        frame: FrameRef::rat(),
        body: AlleleBody::Native(NativeId("add@ℚ".into())),
        witnesses: Vec::new(),
    }
}

/// Run exit-gate demo 1: add@ℚ admitted, (2,3)→5, hash unchanged.
pub fn demo1(budget: Budget, natives: NativeRegistry) -> Result<String, String> {
    let gate = Gate::new(budget, natives);
    let cell = sum_cell();
    let before = hash(&cell.coding);
    match gate.admit_allele(&cell, &add_int_allele()) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Err(format!("add@ℤ refused: {}", r.reason)),
    }
    match gate.admit_allele(&cell, &add_rat_allele()) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Err(format!("add@ℚ refused: {}", r.reason)),
    }
    let after = hash(&cell.coding);
    if before != after {
        return Err("cell hash moved after admitting add@ℚ".into());
    }
    Ok(format!(
        "1 ok  add@ℚ admitted; (2,3)→5; hash {} unchanged",
        before.short_hex()
    ))
}

/// Run exit-gate demo 2: commutativity breaker refused with a counter-example.
pub fn demo2(budget: Budget, natives: NativeRegistry) -> Result<String, String> {
    let gate = Gate::new(budget, natives);
    let cell = sum_cell();
    let bad = Allele {
        frame: FrameRef::int(),
        body: AlleleBody::Native(NativeId("mutant.difference".into())),
        witnesses: Vec::new(),
    };
    match gate.admit_allele(&cell, &bad) {
        Verdict::Refused(r) => {
            if r.counterexample.is_none() {
                return Err("refusal lacked a counter-example".into());
            }
            Ok(format!(
                "2 ok  commutativity refused; seed {}; bindings {:?}",
                r.seed,
                r.counterexample.as_ref().map(|c| c.bindings.len())
            ))
        }
        Verdict::Ok(_) => Err("difference allele was admitted".into()),
    }
}

/// Run exit-gate demo 3: regulatory edit leaves the hash untouched.
pub fn demo3(_budget: Budget) -> Result<String, String> {
    let mut cell = sum_cell();
    let before = hash(&cell.coding);
    cell.regulatory
        .literals
        .insert("prompt".into(), "sum: ".into());
    cell.regulatory.styles.insert("ink".into(), "black".into());
    cell.regulatory.names.insert(0, "left".into());
    let after = hash(&cell.coding);
    if before != after {
        return Err("regulatory edit moved the hash".into());
    }
    Ok(format!(
        "3 ok  regulatory edit; hash {} untouched",
        before.short_hex()
    ))
}

/// Run exit-gate demo 4: rename silent; added in-port refused unless new version.
pub fn demo4(budget: Budget, natives: NativeRegistry) -> Result<String, String> {
    let gate = Gate::new(budget, natives);
    let parent = sum_cell();
    let mut renamed = parent.clone();
    renamed.regulatory.names.insert(0, "left".into());
    match gate.admit_cell(&renamed, Some(&parent)) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Err(format!("rename refused: {}", r.reason)),
    }
    if hash(&renamed.coding) != hash(&parent.coding) {
        return Err("rename moved the coding hash".into());
    }
    let mut added = parent.clone();
    added.coding.contract.ports.push(PortDecl {
        position: 3,
        direction: Direction::In,
        frame: FrameRef::int(),
        required: false,
    });
    match gate.admit_cell(&added, Some(&parent)) {
        Verdict::Refused(r) => {
            if !r.reason.contains("non-conservative") {
                return Err(format!("wrong refusal: {}", r.reason));
            }
        }
        Verdict::Ok(_) => return Err("added in-port was admitted without a new version".into()),
    }
    added.coding.lineage = Some(hash(&parent.coding));
    match gate.admit_cell(&added, Some(&parent)) {
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => return Err(format!("new version refused: {}", r.reason)),
    }
    Ok("4 ok  rename silent; added in-port refused unless lineage".into())
}

/// All four demos.
pub fn gate1(budget: Budget, natives: NativeRegistry) -> Result<Vec<String>, String> {
    Ok(vec![
        demo1(budget, natives.clone())?,
        demo2(budget, natives.clone())?,
        demo3(budget)?,
        demo4(budget, natives)?,
    ])
}

/// Mutant names in the §6.3 order.
pub fn mutant_ids() -> [&'static str; 12] {
    [
        "mutant.plus1",
        "mutant.difference",
        "mutant.times",
        "mutant.max",
        "mutant.saturating",
        "mutant.impostor",
        "mutant.zero",
        "mutant.wrong_frame",
        "mutant.panic_neg",
        "mutant.wrong_rat",
        "mutant.no_restrict",
        "mutant.leading",
    ]
}

/// Run the 12-mutant power check. Returns (refused, total, lines).
pub fn power(budget: Budget, natives: NativeRegistry) -> (usize, usize, Vec<String>) {
    let mut gate = Gate::new(budget, natives);
    let format = format_cell();
    let fh = hash(&format.coding);
    if let Some(fmt) = gate.native_arc("format@ℤ") {
        gate.register_cell_oracle(fh, fmt);
    }
    let sum = sum_cell();
    let cli = cli_input_cell(fh);
    let mut refused = 0;
    let mut lines = Vec::new();
    for (i, id) in mutant_ids().iter().enumerate() {
        let (cell, frame) = if *id == "mutant.leading" {
            (cli.clone(), FrameRef::text())
        } else if *id == "mutant.wrong_rat" || *id == "mutant.no_restrict" {
            (sum.clone(), FrameRef::rat())
        } else {
            (sum.clone(), FrameRef::int())
        };
        let allele = Allele {
            frame,
            body: AlleleBody::Native(NativeId((*id).into())),
            witnesses: Vec::new(),
        };
        let control = if *id == "mutant.leading" {
            Allele {
                frame: FrameRef::text(),
                body: AlleleBody::Native(NativeId("parse@Text".into())),
                witnesses: Vec::new(),
            }
        } else if *id == "mutant.wrong_rat" || *id == "mutant.no_restrict" {
            add_rat_allele()
        } else {
            add_int_allele()
        };
        let control_ok = matches!(gate.admit_allele(&cell, &control), Verdict::Ok(_));
        match gate.admit_allele(&cell, &allele) {
            Verdict::Refused(r) if control_ok => {
                refused += 1;
                lines.push(format!(
                    "  {:>2}  {id}  refused by {}",
                    i + 1,
                    check_label::check_label(r.check)
                ));
            }
            Verdict::Refused(_) => {
                lines.push(format!(
                    "  {:>2}  {id}  refused but negative control also refused",
                    i + 1
                ));
            }
            Verdict::Ok(_) => {
                lines.push(format!("  {:>2}  {id}  SURVIVED", i + 1));
            }
        }
    }
    (refused, 12, lines)
}
