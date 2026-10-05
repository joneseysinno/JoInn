//! The line every tick prints.

use joinn_visual::print_zoom;

use super::Tick;

/// `tick: level <L> step <t> (k <fraction>), anchor <chart>, rows <n>`, where
/// `rows` counts the table rows the tick wrote.
pub fn tick_line(tick: &Tick, rows: usize) -> String {
    format!(
        "tick: {}, anchor {}, rows {rows}",
        print_zoom(&tick.zoom),
        tick.anchor
    )
}
