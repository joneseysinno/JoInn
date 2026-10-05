//! How far from the anchor any chart reaches, in layout units.

use joinn_visual::{Tables, UNIVERSE_SIZE};

/// The largest chart origin coordinate, in magnitude, plus the universe's
/// width: no chart corner lies farther from the anchor's origin than this.
pub(super) fn chart_extent(tables: &Tables) -> i64 {
    let far = tables
        .chart
        .iter()
        .map(|c| i64::from(c.origin_x).abs().max(i64::from(c.origin_y).abs()))
        .max()
        .unwrap_or(0);
    far + UNIVERSE_SIZE.0.max(UNIVERSE_SIZE.1)
}
