//! Gate 5.1: an unparseable subject refuses the run by name.

use super::run_gate_table;
use super::subject::Subject;
use joinn_gate::GateItem;

#[allow(dead_code)]
pub(crate) fn g51_reads() -> bool {
    match run_gate_table(
        "fixture",
        &[GateItem {
            name: "bad universe",
            check: || true,
            control: |_: &Subject| false,
            control_artifact: "xtask/gate_fixtures/not_a_universe.universe",
        }],
    ) {
        Err(msg) => msg.contains("bad universe"),
        Ok(_) => false,
    }
}
