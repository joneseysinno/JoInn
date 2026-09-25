//! Gate 5.1 table. Wrappers deleted; Links are typed split (§2.4).

use super::mutate::Mutation;
use super::subject::Subject;
use super::{
    g51_frame, g51_frame_control, g51_hosts, g51_hosts_control, g51_tails, g51_tails_control,
    g51_total, g51_total_control,
};
use joinn_gate::GateItem;

/// echo.body coding hash — SwapBinding target for *Members share a frame*.
const ECHO_HASH: &str = "556e785914ff720d908e14f10daa81ac797e0e0259b63a9647c68651a7703f57";

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "Two hosts, one universe",
        check: g51_hosts,
        control: g51_hosts_control,
        control_artifact: "corpus/transcripts/universe.txt",
        opposes: Mutation::SwapLines(2, 3),
    },
    GateItem {
        name: "Tails are out, heads are in",
        check: g51_tails,
        control: g51_tails_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::FlipMark("e0", "calc.sum@2"),
    },
    GateItem {
        name: "Members share a frame",
        check: g51_frame,
        control: g51_frame_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::SwapBinding("units", ECHO_HASH),
    },
    GateItem {
        name: "The boundary is total",
        check: g51_total,
        control: g51_total_control,
        control_artifact: "corpus/phase52/controls/missing_cell.body",
        opposes: Mutation::DropGenome("orphan"),
    },
];

pub(crate) fn gate_five_one_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
