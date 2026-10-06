//! Gate 7.3 table (P73-13: links touch and never cross, order and forms,
//! folded systems touched once).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g73_folded, g73_folded_control, g73_order, g73_order_control, g73_touch, g73_touch_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "A hyperedge touches; it never crosses",
        check: g73_touch,
        control: g73_touch_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::WireAcross("path"),
    },
    GateItem {
        name: "Order is drawn only when declared; the form follows size",
        check: g73_order,
        control: g73_order_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::FlipMark("path", "calc.sum@2"),
    },
    GateItem {
        name: "A folded system is touched once",
        check: g73_folded,
        control: g73_folded_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::FlipMark("e0", "units.scale@0"),
    },
];

pub(crate) fn gate_seven_three_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
