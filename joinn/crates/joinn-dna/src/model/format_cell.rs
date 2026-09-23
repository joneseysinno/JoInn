//! `Format`: ℤ → Text. Exists so CliInput can name another cell by hash.

use crate::formula::{Formula, Law, LawName, Term_, VarId};
use joinn_frame::FrameRef;
use std::collections::BTreeMap;

use super::{
    Allele, AlleleBody, Cell, CodingRegion, Contract, Direction, JoinPolicy, NativeId, PortDecl,
    RegulatoryRegion, Witness,
};
use crate::model::generate_int::generate_int;
use crate::model::generate_text::generate_text;

/// `Format`: ℤ → Text. Exists so CliInput can name another cell by hash.
pub fn format_cell() -> Cell {
    let z = FrameRef::int();
    let t = FrameRef::text();
    let n = Term_::Var(VarId("n".into()));
    let functionality = Law {
        name: LawName("functionality".into()),
        formula: Formula::ForAll {
            vars: vec![(VarId("n".into()), z), (VarId("m".into()), z)],
            body: Box::new(Formula::Implies(
                Box::new(Formula::Eq(n.clone(), Term_::Var(VarId("m".into())))),
                Box::new(Formula::Eq(
                    Term_::SelfAt {
                        out: 1,
                        args: BTreeMap::from([(0, n.clone())]),
                    },
                    Term_::SelfAt {
                        out: 1,
                        args: BTreeMap::from([(0, Term_::Var(VarId("m".into())))]),
                    },
                )),
            )),
        },
    };
    let mut laws = BTreeMap::new();
    laws.insert(functionality.name.clone(), functionality);

    let two = generate_int(2);
    let two_txt = generate_text("2");

    Cell {
        coding: CodingRegion {
            codex: 1,
            frame: z,
            contract: Contract {
                ports: vec![
                    PortDecl {
                        position: 0,
                        direction: Direction::In,
                        frame: z,
                        required: true,
                    },
                    PortDecl {
                        position: 1,
                        direction: Direction::Out,
                        frame: t,
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
                inputs: BTreeMap::from([(0, two)]),
                outputs: BTreeMap::from([(1, two_txt)]),
            }],
            declarations: Vec::new(),
            lineage: None,
            turns: Vec::new(),
        },
        regulatory: RegulatoryRegion {
            names: BTreeMap::from([(0, "n".into()), (1, "text".into())]),
            literals: BTreeMap::new(),
            styles: BTreeMap::new(),
            labels: BTreeMap::new(),
        },
        alleles: vec![Allele {
            frame: z,
            body: AlleleBody::Native(NativeId("format@ℤ".into())),
            witnesses: Vec::new(),
        }],
    }
}
