//! Gate 5.2 table (P52-09: binding item only).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{g52_bind, g52_bind_control};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[GateItem {
    name: "Binding is by store",
    check: g52_bind,
    control: g52_bind_control,
    control_artifact: "corpus/phase5/universe.universe",
    opposes: Mutation::CorruptHash("calc"),
}];

pub(crate) fn gate_five_two_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
