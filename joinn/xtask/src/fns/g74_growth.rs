//! Gate 7.4 item 1: a body grows by its DNA, and every size is true.

use joinn_frame::{Frame, FrameRegistry, IntFrame, Term, Verdict};
use joinn_link::{count_witness, grow, grow_step, respond, transcripts};

use super::forces::corpus_cells;
use super::grow::corpus_system;

/// A system, its inputs, and each step's predicted count, or `None` where the
/// body refuses the input (the transcript ends there).
type Predicted = (&'static str, &'static [i64], &'static [Option<i64>]);

/// Plan 7.4 §3's transcripts in order.
const PREDICTED: &[Predicted] = &[
    ("counting", &[], &[]),
    ("counting", &[1, 1, 1], &[Some(1), Some(2), Some(3)]),
    ("counting", &[1, 1, 3], &[Some(1), Some(2), None]),
    ("counting", &[1; 12], TWELVE),
    ("adding", &[], &[]),
    ("adding", &[2, 3, 4], &[Some(2), Some(5), Some(9)]),
    (
        "adding",
        &[5, -2, 0, 7],
        &[Some(5), Some(3), Some(3), Some(10)],
    ),
    ("adding", &[1, 1, 1], &[Some(1), Some(2), Some(3)]),
    ("adding", &[1, 1, 3], &[Some(1), Some(2), Some(5)]),
    ("adding", &[1; 12], TWELVE),
];

const TWELVE: &[Option<i64>] = &[
    Some(1),
    Some(2),
    Some(3),
    Some(4),
    Some(5),
    Some(6),
    Some(7),
    Some(8),
    Some(9),
    Some(10),
    Some(11),
    Some(12),
];

/// Every §3 transcript on both systems (the table above is the systems'
/// transcripts, in order) gives its predicted counts and refusals; every size
/// starts at 0, and `count_witness` agrees with the engine at every step.
/// One line per transcript: `growth <system> <inputs>: counts 0, …`.
pub(crate) fn g74_growth() -> bool {
    let frames = FrameRegistry::phase1();
    let cells = match corpus_cells(&frames) {
        Ok(c) => c,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut ok = true;
    for name in ["counting", "adding"] {
        let s = match corpus_system(name) {
            Ok(s) => s,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let table: Vec<Vec<i64>> = PREDICTED
            .iter()
            .filter(|(n, _, _)| *n == name)
            .map(|(_, inputs, _)| inputs.to_vec())
            .collect();
        if table != transcripts(s.accepts) {
            println!("growth {name}: the predicted table is not §3's transcripts");
            ok = false;
        }
    }
    for (name, inputs, want) in PREDICTED {
        let s = match corpus_system(name) {
            Ok(s) => s,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let mut grown = match grow(&s.system, &s.contacts, &cells, &[]) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => {
                println!("growth {name}: {}", r.reason);
                return false;
            }
        };
        let mut counts = Vec::new();
        let mut got = Vec::new();
        let mut agrees = true;
        for step in 0..=inputs.len() {
            if let Some(v) = step.checked_sub(1).and_then(|i| inputs.get(i)) {
                let value = match IntFrame::new().canonicalize(Term::int(*v)) {
                    Verdict::Ok(v) => v,
                    Verdict::Refused(r) => {
                        println!("growth {name}: {}", r.reason);
                        return false;
                    }
                };
                match grow_step(&grown, &value) {
                    Verdict::Ok(g) => grown = g,
                    Verdict::Refused(_) => {
                        counts.push(format!("then {v} refused"));
                        got.push(None);
                        break;
                    }
                }
            }
            let (response, witness) = match (
                respond(&grown, &cells, &frames),
                count_witness(grown.inputs()),
            ) {
                (Verdict::Ok(r), Verdict::Ok(w)) => (r.print_term(), w.print_term()),
                (Verdict::Refused(r), _) | (_, Verdict::Refused(r)) => {
                    println!("growth {name}: {}", r.reason);
                    return false;
                }
            };
            agrees &= response == witness;
            if step == 0 && response != "0" {
                println!("growth {name}: size 0 shows {response}; acceptance is 0");
                ok = false;
            }
            if step > 0 {
                got.push(response.parse::<i64>().ok());
            }
            counts.push(response);
        }
        let printed: Vec<String> = inputs.iter().map(i64::to_string).collect();
        let printed = if printed.is_empty() {
            "(none)".to_owned()
        } else {
            printed.join(" ")
        };
        println!(
            "growth {name} {printed}: counts {}; witness {}",
            counts.join(", "),
            if agrees { "agrees" } else { "differs" }
        );
        if got.as_slice() != *want || !agrees {
            println!(
                "growth {name} {printed}: predicted {want:?}; acceptance is the prediction and the witness"
            );
            ok = false;
        }
    }
    ok
}
