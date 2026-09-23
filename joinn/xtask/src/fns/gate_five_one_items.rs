//! Gate 5.1 table.

use super::{
    g51_cross, g51_cross_control, g51_hash, g51_hash_control, g51_home, g51_home_control,
    g51_hosts, g51_hosts_control, g51_lock, g51_lock_control, g51_reads, g51_reads_control,
    g51_revoke, g51_revoke_control, g51_total, g51_total_control, g51_typed, g51_typed_control,
};
use joinn_gate::GateItem;

const ITEMS: &[GateItem] = &[
    GateItem {
        name: "Something crosses",
        check: g51_cross,
        control: g51_cross_control,
        control_artifact: "corpus/phase51/controls/unlinked.universe",
    },
    GateItem {
        name: "Two hosts, one universe",
        check: g51_hosts,
        control: g51_hosts_control,
        control_artifact: "corpus/transcripts/universe.txt",
    },
    GateItem {
        name: "Bodies are bound by hash",
        check: g51_hash,
        control: g51_hash_control,
        control_artifact: "corpus/phase5/controls/wrong_hash.universe",
    },
    GateItem {
        name: "Links are typed",
        check: g51_typed,
        control: g51_typed_control,
        control_artifact: "corpus/phase5/controls/wrong_direction.universe",
    },
    GateItem {
        name: "The boundary is total",
        check: g51_total,
        control: g51_total_control,
        control_artifact: "corpus/phase51/controls/missing_cell.body",
    },
    GateItem {
        name: "Revocation stops delivery",
        check: g51_revoke,
        control: g51_revoke_control,
        control_artifact: "corpus/phase5/universe.universe",
    },
    GateItem {
        name: "A refusal stays home",
        check: g51_home,
        control: g51_home_control,
        control_artifact: "corpus/phase5/controls/inner_reason.txt",
    },
    GateItem {
        name: "Every control reads its artifact",
        check: g51_reads,
        control: g51_reads_control,
        control_artifact: "xtask/gate_fixtures/insensitive.rs",
    },
    GateItem {
        name: "The lock is this run",
        check: g51_lock,
        control: g51_lock_control,
        control_artifact: "xtask/gate_fixtures/off_by_one.lock",
    },
];

pub(crate) fn gate_five_one_items() -> &'static [GateItem] {
    ITEMS
}
