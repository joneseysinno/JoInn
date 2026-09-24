//! Legacy gate table: run checks only; controls are not graded.

use joinn_gate::GateItem;

pub(crate) fn run_legacy_table(phase: &str, items: &[GateItem]) -> Result<(u32, u32), String> {
    let mut n = 0u32;
    let total = items.len() as u32;
    for (i, item) in items.iter().enumerate() {
        let index = (i + 1) as u32;
        if (item.check)() {
            println!("{index} ok  {}", item.name);
            n += 1;
        } else {
            println!("{index} fail  {}", item.name);
        }
    }
    println!("{phase}: {n}/{total} legacy");
    Ok((n, total))
}
