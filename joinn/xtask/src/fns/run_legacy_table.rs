//! Legacy gate table: run checks only; controls are not graded.

use joinn_gate::GateItem;

use super::gate_row::gate_row;
use super::once::say;

pub(crate) fn run_legacy_table(phase: &str, items: &[GateItem]) -> Result<(u32, u32), String> {
    let mut n = 0u32;
    let total = items.len() as u32;
    for (i, item) in items.iter().enumerate() {
        let ok = (item.check)();
        say(&gate_row(i + 1, item.name, ok));
        if ok {
            n += 1;
        }
    }
    say(&format!("{phase}: {n}/{total} legacy"));
    Ok((n, total))
}
