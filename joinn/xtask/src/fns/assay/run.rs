//! Load the corpus and print one assay report.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::assay as derive;
use joinn_link::{
    BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, bind, print_assay,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;

/// Assay one `.body` or `.universe`. A body is wrapped as alias `body`.
pub(crate) fn assay(path: &str) -> Result<String, String> {
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let corpus = root.join("corpus");
    let mut cell_paths = Vec::new();
    walk_cells(&corpus, &mut cell_paths)?;
    let mut cells = BTreeMap::new();
    for cell_path in &cell_paths {
        let text = fs::read_to_string(cell_path).map_err(|e| e.to_string())?;
        let cell = match parse_cell(&text, &frames) {
            Verdict::Ok(cell) => cell,
            Verdict::Refused(_) => continue,
        };
        let id = hash(&cell.coding);
        cells.entry(id).or_insert(cell);
    }
    let mut dirs = vec![corpus];
    let mut body_paths = Vec::new();
    while let Some(dir) = dirs.pop() {
        let read = fs::read_dir(&dir).map_err(|e| e.to_string())?;
        let mut names = Vec::new();
        for ent in read {
            names.push(ent.map_err(|e| e.to_string())?.path());
        }
        names.sort();
        for name in names {
            if name.is_dir() {
                if name.file_name().is_some_and(|n| n == "counterfeit") {
                    continue;
                }
                dirs.push(name);
            } else if name.extension().is_some_and(|e| e == "body") {
                body_paths.push(name);
            }
        }
    }
    body_paths.sort();
    let mut store = BodyStore::new();
    for body_path in &body_paths {
        let text = fs::read_to_string(body_path).map_err(|e| e.to_string())?;
        let body = match parse_body(&text, &frames) {
            Verdict::Ok(body) => body,
            Verdict::Refused(_) => continue,
        };
        let source = body_path.display().to_string();
        match store.insert(body, cells.clone(), &source) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    let given = PathBuf::from(path);
    let full = if given.is_absolute() && given.exists() {
        given
    } else if root.join(&given).exists() {
        root.join(&given)
    } else if root.join("corpus").join(&given).exists() {
        root.join("corpus").join(given)
    } else {
        return Err(format!("no such artifact {path}"));
    };
    let text = fs::read_to_string(&full).map_err(|e| e.to_string())?;
    let universe = if full.extension().is_some_and(|e| e == "body") {
        let body = match parse_body(&text, &frames) {
            Verdict::Ok(body) => body,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let id = hash(&body.coding);
        match store.insert(body, cells, &full.display().to_string()) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
        Universe {
            coding: UniverseCoding {
                codex: 1,
                bodies: vec![BodyBinding {
                    hash: id,
                    alias: "body".to_string(),
                }],
                links: Vec::new(),
                cross_wires: Vec::new(),
                grants: BTreeMap::new(),
                lenses: Vec::new(),
            },
            regulatory: UniverseRegulatory::default(),
        }
    } else if full.extension().is_some_and(|e| e == "universe") {
        match joinn_link::parse_universe(&text) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => return Err(r.reason),
        }
    } else {
        return Err(format!(
            "{} is not a body or universe; acceptance is a .body or .universe path",
            full.display()
        ));
    };
    let bound = match bind(&universe, &store) {
        Verdict::Ok(bound) => bound,
        Verdict::Refused(r) => return Err(r.reason),
    };
    match derive(&universe, &bound) {
        Verdict::Ok(report) => Ok(print_assay(&report)),
        Verdict::Refused(r) => Err(r.reason),
    }
}

#[cfg(test)]
mod tests {
    use super::assay;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::walk_cells::walk_cells;
    use crate::fns::workspace_root::workspace_root;
    use joinn_dna::{hash, parse_body, parse_cell};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use joinn_link::assay as derive;
    use joinn_link::{
        BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, bind, print_assay,
    };
    use std::collections::BTreeMap;
    use std::fs;

    const CALCULATOR: &str = "\
assay body
regions: body 1
islands: 1
loops: 2
filled: outside →body.cli_a@0→ body →body.sum@2→ outside by body.cli_a roundtrip
filled: outside →body.cli_b@0→ body →body.sum@2→ outside by body.cli_b roundtrip
not measured: 0
H₀: 1
H₁: 0
H₂: 0
euler: V 2 − E 3 + F 2 = 1 = 1 − 0 + 0
";

    fn show(label: &str, text: &str) {
        eprintln!("===== {label} =====\n{text}");
    }

    fn corpus() -> (BTreeMap<Hash, joinn_dna::Cell>, BodyStore) {
        let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
        let frames = FrameRegistry::phase1();
        let corpus = root.join("corpus");
        let mut cell_paths = Vec::new();
        walk_cells(&corpus, &mut cell_paths).unwrap_or_else(|e| panic!("{e}"));
        let mut cells = BTreeMap::new();
        for cell_path in &cell_paths {
            let text = fs::read_to_string(cell_path).unwrap_or_else(|e| panic!("{e}"));
            let cell = match parse_cell(&text, &frames) {
                Verdict::Ok(cell) => cell,
                Verdict::Refused(_) => continue,
            };
            cells.entry(hash(&cell.coding)).or_insert(cell);
        }
        let mut dirs = vec![corpus];
        let mut body_paths = Vec::new();
        while let Some(dir) = dirs.pop() {
            let read = fs::read_dir(&dir).unwrap_or_else(|e| panic!("{e}"));
            let mut names = Vec::new();
            for ent in read {
                names.push(ent.unwrap_or_else(|e| panic!("{e}")).path());
            }
            names.sort();
            for name in names {
                if name.is_dir() {
                    if name.file_name().is_some_and(|n| n == "counterfeit") {
                        continue;
                    }
                    dirs.push(name);
                } else if name.extension().is_some_and(|e| e == "body") {
                    body_paths.push(name);
                }
            }
        }
        body_paths.sort();
        let mut store = BodyStore::new();
        for body_path in &body_paths {
            let text = fs::read_to_string(body_path).unwrap_or_else(|e| panic!("{e}"));
            let body = match parse_body(&text, &frames) {
                Verdict::Ok(body) => body,
                Verdict::Refused(_) => continue,
            };
            let source = body_path.display().to_string();
            match store.insert(body, cells.clone(), &source) {
                Verdict::Ok(_) => {}
                Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        (cells, store)
    }

    fn read_subject(rel: &str) -> Subject {
        let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
        let path = root.join("corpus").join(rel);
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
        parse_subject(rel, &text).unwrap_or_else(|e| panic!("{e}"))
    }

    fn hex_cell(rel: &str) -> String {
        let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
        let text =
            fs::read_to_string(root.join("corpus").join(rel)).unwrap_or_else(|e| panic!("{e}"));
        let cell = match parse_cell(&text, &FrameRegistry::phase1()) {
            Verdict::Ok(cell) => cell,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        hash(&cell.coding).to_hex()
    }

    fn hex_body(rel: &str) -> String {
        let Subject::Body(body) = read_subject(rel) else {
            panic!("body");
        };
        hash(&body.coding).to_hex()
    }

    fn wrap(body: &joinn_dna::Body) -> Universe {
        Universe {
            coding: UniverseCoding {
                codex: 1,
                bodies: vec![BodyBinding {
                    hash: hash(&body.coding),
                    alias: "body".to_string(),
                }],
                links: Vec::new(),
                cross_wires: Vec::new(),
                grants: BTreeMap::new(),
                lenses: Vec::new(),
            },
            regulatory: UniverseRegulatory::default(),
        }
    }

    fn printed(
        store: &mut BodyStore,
        cells: &BTreeMap<Hash, joinn_dna::Cell>,
        subject: &Subject,
    ) -> String {
        let universe = match subject {
            Subject::Body(body) => {
                match store.insert(body.clone(), cells.clone(), "assay mutant") {
                    Verdict::Ok(_) => {}
                    Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
                    Verdict::Refused(r) => panic!("{}", r.reason),
                }
                wrap(body)
            }
            Subject::Universe(u) => u.clone(),
            _ => panic!("body or universe"),
        };
        let bound = match bind(&universe, store) {
            Verdict::Ok(bound) => bound,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let report = match derive(&universe, &bound) {
            Verdict::Ok(report) => report,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        print_assay(&report)
    }

    fn must(path: &str) -> String {
        match assay(path) {
            Ok(text) => text,
            Err(e) => panic!("{path}: {e}"),
        }
    }

    #[test]
    fn section_3_2_reports() {
        let calc = must("phase2/calculator.body");
        show("calculator.body", &calc);
        assert_eq!(calc, CALCULATOR);

        let universe = must("phase5/universe.universe");
        show("universe.universe", &universe);
        assert!(universe.contains("regions: calc 1\n"), "{universe}");
        assert!(universe.contains("regions: units 1\n"), "{universe}");
        assert!(universe.contains("islands: 1\n"), "{universe}");
        assert!(universe.contains("loops: 3\n"), "{universe}");
        assert!(universe.contains("by calc.cli_a roundtrip\n"), "{universe}");
        assert!(universe.contains("by calc.cli_b roundtrip\n"), "{universe}");
        assert!(universe.contains("by frame ℤ\n"), "{universe}");
        assert_eq!(universe.matches("filled:").count(), 3, "{universe}");
        assert!(universe.contains("H₁: 0\n"), "{universe}");
        assert!(universe.contains("H₂: 0\n"), "{universe}");
        assert!(
            universe.contains("euler: V 3 − E 5 + F 3 = 1 = 1 − 0 + 0\n"),
            "{universe}"
        );

        let ask = must("phase52/adversary/ask.universe");
        show("ask.universe", &ask);
        assert!(
            ask.contains("regions: lookup 2 {question} {answer}\n"),
            "{ask}"
        );
        assert!(ask.contains("regions: units 1\n"), "{ask}");
        assert!(ask.contains("loops: 4\n"), "{ask}");
        assert_eq!(ask.matches("filled:").count(), 4, "{ask}");
        assert_eq!(ask.matches("by frame ℤ").count(), 4, "{ask}");
        assert!(ask.contains("H₁: 0\n"), "{ask}");
        assert!(
            ask.contains("euler: V 4 − E 7 + F 4 = 1 = 1 − 0 + 0\n"),
            "{ask}"
        );

        let loop_u = must("phase4/loop.universe");
        show("loop.universe", &loop_u);
        assert!(loop_u.contains("regions: calc 1\n"), "{loop_u}");
        assert!(loop_u.contains("regions: fmt 1\n"), "{loop_u}");
        assert!(loop_u.contains("loops: 1\n"), "{loop_u}");
        assert!(loop_u.contains("by calc.cli_a roundtrip\n"), "{loop_u}");
        assert_eq!(loop_u.matches("filled:").count(), 1, "{loop_u}");
        assert!(loop_u.contains("H₁: 0\n"), "{loop_u}");
        assert!(
            loop_u.contains("euler: V 3 − E 3 + F 1 = 1 = 1 − 0 + 0\n"),
            "{loop_u}"
        );

        let (cells, mut store) = corpus();
        let open_hex = hex_cell("phase4/cli_input_open.cell");
        let open_static: &'static str = Box::leak(open_hex.into_boxed_str());
        let calc_subject = read_subject("phase2/calculator.body");
        let swapped_b = match mutate(&calc_subject, &Mutation::SwapCell("cli_b", open_static)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let one = printed(&mut store, &cells, &swapped_b);
        show("SwapCell cli_b", &one);
        assert!(
            one.contains(
                "filled: outside →body.cli_a@0→ body →body.sum@2→ outside by body.cli_a roundtrip\n"
            ),
            "{one}"
        );
        assert!(
            one.contains("open: outside →body.cli_b@0→ body →body.sum@2→ outside\n"),
            "{one}"
        );
        assert_eq!(one.matches("filled:").count(), 1, "{one}");
        assert!(one.contains("H₁: 1\n"), "{one}");
        assert!(
            one.contains("euler: V 2 − E 3 + F 1 = 0 = 1 − 1 + 0\n"),
            "{one}"
        );

        let swapped_both = match mutate(&swapped_b, &Mutation::SwapCell("cli_a", open_static)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let both = printed(&mut store, &cells, &swapped_both);
        show("SwapCell both", &both);
        assert_eq!(both.matches("filled:").count(), 0, "{both}");
        assert!(
            both.contains("open: outside →body.cli_a@0→ body →body.sum@2→ outside\n"),
            "{both}"
        );
        assert!(
            both.contains("open: outside →body.cli_b@0→ body →body.sum@2→ outside\n"),
            "{both}"
        );
        assert!(both.contains("H₁: 2\n"), "{both}");
        assert!(
            both.contains("euler: V 2 − E 3 + F 0 = -1 = 1 − 2 + 0\n"),
            "{both}"
        );

        let twin_hex = hex_body("phase4/fmt_twin.body");
        let twin_static: &'static str = Box::leak(twin_hex.into_boxed_str());
        let loop_subject = read_subject("phase4/loop.universe");
        let twin = match mutate(&loop_subject, &Mutation::SwapBinding("fmt", twin_static)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let twin_text = printed(&mut store, &cells, &twin);
        show("SwapBinding fmt", &twin_text);
        assert_eq!(twin_text.matches("filled:").count(), 0, "{twin_text}");
        assert_eq!(twin_text.matches("open:").count(), 1, "{twin_text}");
        assert!(twin_text.contains("H₁: 1\n"), "{twin_text}");
        assert!(twin_text.contains("loops: 1\n"), "{twin_text}");
        assert!(
            twin_text.contains("euler: V 3 − E 3 + F 0 = 0 = 1 − 1 + 0\n"),
            "{twin_text}"
        );
    }
}
