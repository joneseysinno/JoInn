//! Hand-built `Sum` coding region used by tests and the gate demos.

use crate::formula::{Formula, Law, LawName, Term_, VarId};
use joinn_frame::{FrameRef, OpName};
use std::collections::BTreeMap;

use super::{
    Allele, AlleleBody, Cell, CodingRegion, Contract, Direction, JoinPolicy, NativeId, PortDecl,
    RegulatoryRegion, Witness,
};
use crate::model::generate_int::generate_int;

/// Hand-built `Sum` coding region used by tests and the gate demos.
pub fn sum_cell() -> Cell {
    let z = FrameRef::int();
    let a = Term_::Var(VarId("a".into()));
    let b = Term_::Var(VarId("b".into()));
    let c = Term_::Var(VarId("c".into()));
    let zero = Term_::FrameOp {
        frame: z,
        op: OpName("zero".into()),
        args: Vec::new(),
    };
    let self_at = |args: BTreeMap<u32, Term_>| Term_::SelfAt { out: 2, args };

    let identity = Law {
        name: LawName("identity".into()),
        formula: Formula::ForAll {
            vars: vec![(VarId("a".into()), z)],
            body: Box::new(Formula::Eq(
                self_at(BTreeMap::from([(0, a.clone()), (1, zero)])),
                a.clone(),
            )),
        },
    };
    let commutative = Law {
        name: LawName("commutative".into()),
        formula: Formula::ForAll {
            vars: vec![(VarId("a".into()), z), (VarId("b".into()), z)],
            body: Box::new(Formula::Eq(
                self_at(BTreeMap::from([(0, a.clone()), (1, b.clone())])),
                self_at(BTreeMap::from([(0, b.clone()), (1, a.clone())])),
            )),
        },
    };
    let inner_ab = self_at(BTreeMap::from([(0, a.clone()), (1, b.clone())]));
    let inner_bc = self_at(BTreeMap::from([(0, b.clone()), (1, c.clone())]));
    let associative = Law {
        name: LawName("associative".into()),
        formula: Formula::ForAll {
            vars: vec![
                (VarId("a".into()), z),
                (VarId("b".into()), z),
                (VarId("c".into()), z),
            ],
            body: Box::new(Formula::Eq(
                self_at(BTreeMap::from([(0, inner_ab), (1, c.clone())])),
                self_at(BTreeMap::from([(0, a), (1, inner_bc)])),
            )),
        },
    };

    let two = generate_int(2);
    let three = generate_int(3);
    let five = generate_int(5);

    let mut laws = BTreeMap::new();
    laws.insert(identity.name.clone(), identity);
    laws.insert(commutative.name.clone(), commutative);
    laws.insert(associative.name.clone(), associative);

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
                        direction: Direction::In,
                        frame: z,
                        required: true,
                    },
                    PortDecl {
                        position: 2,
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
                inputs: BTreeMap::from([(0, two), (1, three)]),
                outputs: BTreeMap::from([(2, five)]),
            }],
            declarations: Vec::new(),
            lineage: None,
            turns: Vec::new(),
        },
        regulatory: RegulatoryRegion {
            names: BTreeMap::from([(0, "a".into()), (1, "b".into()), (2, "result".into())]),
            literals: BTreeMap::new(),
            styles: BTreeMap::new(),
            labels: BTreeMap::from([
                (0, "first addend".into()),
                (1, "second addend".into()),
                (2, "sum".into()),
            ]),
        },
        alleles: vec![Allele {
            frame: z,
            body: AlleleBody::Native(NativeId("add@ℤ".into())),
            witnesses: Vec::new(),
        }],
    }
}
