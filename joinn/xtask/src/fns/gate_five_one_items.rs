//! Gate 5.1 table.

use super::{
    g51_cross, g51_cross_control, g51_hash, g51_hash_control, g51_home, g51_home_control,
    g51_hosts, g51_hosts_control, g51_lock, g51_lock_control, g51_revoke, g51_revoke_control,
    g51_total, g51_total_control, g51_typed, g51_typed_control,
};
use super::mutate::Mutation;
use super::subject::Subject;
use joinn_gate::GateItem;

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "Something crosses",
        check: g51_cross,
        control: g51_cross_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::DropLink("e0"),
    },
    GateItem {
        name: "Two hosts, one universe",
        check: g51_hosts,
        control: g51_hosts_control,
        control_artifact: "corpus/transcripts/universe.txt",
        opposes: Mutation::SwapLines(2, 3),
    },
    GateItem {
        name: "Bodies are bound by hash",
        check: g51_hash,
        control: g51_hash_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::CorruptHash("calc"),
    },
    GateItem {
        name: "Links are typed",
        check: g51_typed,
        control: g51_typed_control,
        control_artifact: "corpus/phase5/universe.universe",
        opposes: Mutation::FlipMark("e0", "calc.sum@2"),
    },
    GateItem {
        name: "The boundary is total",
        check: g51_total,
        control: g51_total_control,
        control_artifact: "corpus/phase51/controls/missing_cell.body",
        opposes: Mutation::DropGenome("orphan"),
    },
    GateItem {
        name: "Revocation stops delivery",
        check: g51_revoke,
        control: g51_revoke_control,
        control_artifact: "corpus/phase5/ordered.universe",
        opposes: Mutation::DropLink("path"),
    },
    GateItem {
        name: "A refusal stays home",
        check: g51_home,
        control: g51_home_control,
        control_artifact: "corpus/phase5/controls/inner_reason.txt",
        opposes: Mutation::Replace("e0"),
    },
    GateItem {
        name: "The lock is this run",
        check: g51_lock,
        control: g51_lock_control,
        control_artifact: "xtask/gate_fixtures/off_by_one.lock",
        opposes: Mutation::SetScore(concat!("phase ", "5"), 8, 8),
    },
];

pub(crate) fn gate_five_one_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}
