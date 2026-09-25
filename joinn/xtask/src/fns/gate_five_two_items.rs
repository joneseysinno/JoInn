//! Gate 5.2 table (P52-12: binding, refusal report, host knows no ids).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g52_bind, g52_bind_control, g52_ids, g52_ids_control, g52_refusal, g52_refusal_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "Binding is by store",
        check: g52_bind,
        control: g52_bind_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::CorruptHash("calc"),
    },
    GateItem {
        name: "A refusal is a report",
        check: g52_refusal,
        control: g52_refusal_control,
        control_artifact: "corpus/transcripts/universe.txt",
        opposes: Mutation::DropLine(1),
    },
    GateItem {
        name: "The host knows no ids",
        check: g52_ids,
        control: g52_ids_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::DropGrant("path"),
    },
];

pub(crate) fn gate_five_two_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
