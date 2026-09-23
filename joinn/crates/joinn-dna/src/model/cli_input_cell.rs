//! `CliInput`: Text → ℤ, with `parse(format n) = n` naming Format by hash.

use crate::formula::{Formula, Law, LawName, Term_, VarId};
use joinn_frame::{FrameRef, Hash};
use std::collections::BTreeMap;

use super::{
    Allele, AlleleBody, Cell, CodingRegion, Contract, Direction, JoinPolicy, NativeId, PortDecl,
    RegulatoryRegion, Witness,
};
use crate::model::generate_int::generate_int;
use crate::model::generate_text::generate_text;

/// `CliInput`: Text → ℤ, with `parse(format n) = n` naming Format by hash.
pub fn cli_input_cell(format_hash: Hash) -> Cell {
    let z = FrameRef::int();
    let t = FrameRef::text();
    let n = Term_::Var(VarId("n".into()));
    let s = Term_::Var(VarId("s".into()));
    let formatted = Term_::CellAt {
        cell: format_hash,
        out: 1,
        args: BTreeMap::from([(0, n.clone())]),
    };
    let parsed = Term_::SelfAt {
        out: 1,
        args: BTreeMap::from([(0, formatted)]),
    };
    let roundtrip = Law {
        name: LawName("roundtrip".into()),
        formula: Formula::ForAll {
            vars: vec![(VarId("n".into()), z)],
            body: Box::new(Formula::Eq(parsed, n.clone())),
        },
    };
    let canonical = Law {
        name: LawName("canonical".into()),
        formula: Formula::ForAll {
            vars: vec![(VarId("s".into()), t), (VarId("n".into()), z)],
            body: Box::new(Formula::Implies(
                Box::new(Formula::Eq(
                    Term_::SelfAt {
                        out: 1,
                        args: BTreeMap::from([(0, s.clone())]),
                    },
                    n.clone(),
                )),
                Box::new(Formula::Eq(
                    Term_::CellAt {
                        cell: format_hash,
                        out: 1,
                        args: BTreeMap::from([(0, n)]),
                    },
                    s,
                )),
            )),
        },
    };
    let mut laws = BTreeMap::new();
    laws.insert(roundtrip.name.clone(), roundtrip);
    laws.insert(canonical.name.clone(), canonical);

    let two_txt = generate_text("2");
    let two = generate_int(2);

    Cell {
        coding: CodingRegion {
            codex: 1,
            frame: t,
            contract: Contract {
                ports: vec![
                    PortDecl {
                        position: 0,
                        direction: Direction::In,
                        frame: t,
                        required: true,
                    },
                    PortDecl {
                        position: 1,
                        direction: Direction::Out,
                        frame: z,
                        required: true,
                    },
                ],
                retired: Vec::new(),
                join_policy: JoinPolicy::Refuse,
                require: BTreeMap::new(),
                ensure: BTreeMap::new(),
            },
            laws,
            founding: vec![Witness {
                inputs: BTreeMap::from([(0, two_txt)]),
                outputs: BTreeMap::from([(1, two)]),
            }],
            declarations: Vec::new(),
            lineage: None,
            turns: Vec::new(),
        },
        regulatory: RegulatoryRegion {
            names: BTreeMap::from([(0, "line".into()), (1, "value".into())]),
            literals: BTreeMap::from([("prompt".into(), "a: ".into())]),
            styles: BTreeMap::new(),
            labels: BTreeMap::new(),
        },
        alleles: vec![Allele {
            frame: t,
            body: AlleleBody::Native(NativeId("parse@Text".into())),
            witnesses: Vec::new(),
        }],
    }
}
