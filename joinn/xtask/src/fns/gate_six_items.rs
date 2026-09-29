//! Gate 6 table (P6-13: owners, the picture's rows, a click and a stale pick).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g6_click, g6_click_control, g6_owners, g6_owners_control, g6_picture, g6_picture_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "Every pixel has one owner, and both pickers name it",
        check: g6_owners,
        control: g6_owners_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: Mutation::DropWire("cli_b@1", "sum@1"),
    },
    GateItem {
        name: "What the engine computes is a row the picture shows",
        check: g6_picture,
        control: g6_picture_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: Mutation::DropGenome("sum"),
    },
    GateItem {
        name: "A click names an address; a stale click is refused",
        check: g6_click,
        control: g6_click_control,
        control_artifact: "corpus/phase2/calculator.body",
        opposes: Mutation::DropGenome("cli_b"),
    },
];

pub(crate) fn gate_six_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
