//! Gate 7 table (P7-15: two forms, order-free combine, cells in contact).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g7_forms, g7_forms_control, g7_order_free, g7_order_free_control, g7_picture,
    g7_picture_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "Two forms, one truth",
        check: g7_forms,
        control: g7_forms_control,
        control_artifact: "corpus/phase7/calculator.contact",
        opposes: Mutation::SwapResponse(
            "sum",
            "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7",
        ),
    },
    GateItem {
        name: "Combine is order-free, and it fits what it reaches",
        check: g7_order_free,
        control: g7_order_free_control,
        control_artifact: "corpus/phase7/calculator.contact",
        opposes: Mutation::ShiftMember("sum", "cli_a@1", 0),
    },
    GateItem {
        name: "The body is drawn as cells in contact",
        check: g7_picture,
        control: g7_picture_control,
        control_artifact: "corpus/phase7/calculator.contact",
        opposes: Mutation::DropForce("sum"),
    },
];

pub(crate) fn gate_seven_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
