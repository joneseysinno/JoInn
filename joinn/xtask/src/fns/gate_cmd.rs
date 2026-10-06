//! `cargo xtask gate all | <phase> [--item <n>]`.

use super::gate_item::gate_item;
use super::gate_table::phase_gates;
use super::{gate_all, require_full};

/// `all` runs every gate and writes the lock; a phase runs its whole table;
/// `--item n` runs one item and exits 1 when it fails.
pub(crate) fn gate_cmd(args: Vec<String>) -> Result<(), String> {
    let gates = phase_gates();
    let usage = || {
        let phases: Vec<&str> = gates.iter().map(|g| g.arg).collect();
        format!(
            "usage: cargo xtask gate all|{} [--item <n>]",
            phases.join("|")
        )
    };
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    if args == ["all"] {
        return gate_all();
    }
    let Some(gate) = args
        .first()
        .and_then(|a| gates.iter().find(|g| g.arg == *a))
    else {
        return Err(usage());
    };
    match args[1..] {
        [] => (gate.run)(gate.label).and_then(require_full),
        ["--item", n] => {
            let n: usize = n
                .parse()
                .map_err(|_| format!("--item {n}: acceptance is an item number"))?;
            if gate_item(gate.label, gate.table, n)? {
                Ok(())
            } else {
                Err(format!("{}: item {n} failed", gate.label))
            }
        }
        _ => Err(usage()),
    }
}
