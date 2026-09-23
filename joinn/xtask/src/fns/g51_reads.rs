//! Gate 5.1 item 8: an ignoring control fails the run by name.

use super::run_gate_table;
use joinn_gate::GateItem;

pub(crate) fn g51_reads() -> bool {
    match run_gate_table(
        "fixture",
        &[GateItem {
            name: "ignores its bytes",
            check: super::g51_cross,
            control: super::ignores_bytes::ignores_bytes,
            control_artifact: "xtask/gate_fixtures/insensitive.rs",
        }],
    ) {
        Err(msg) => msg.contains("ignores its bytes") && msg.contains("does not read its artifact"),
        Ok(_) => false,
    }
}
