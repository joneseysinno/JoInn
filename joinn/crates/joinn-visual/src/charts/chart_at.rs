//! The deepest chart whose rectangle holds a point, decided exactly.

use super::UniverseLayout;
use crate::camera::ChartId;

impl UniverseLayout {
    /// The point is `p / den` root-chart layout units (`den > 0`). Rectangles
    /// are half-open. When no chart below the root holds it, the root does.
    pub fn chart_at(&self, p: (i128, i128), den: i128) -> ChartId {
        let holds = |origin: (i64, i64), size: (i64, i64)| {
            let inside = |o: i64, s: i64, q: i128| {
                i128::from(o) * den <= q && q < (i128::from(o) + i128::from(s)) * den
            };
            inside(origin.0, size.0, p.0) && inside(origin.1, size.1, p.1)
        };
        let mut here = 0usize;
        loop {
            let next = self.charts.iter().enumerate().find(|(i, c)| {
                *i != here && c.parent.0 as usize == here && holds(c.root_origin, c.size)
            });
            match next {
                Some((i, _)) => here = i,
                None => return ChartId(u32::try_from(here).unwrap_or(0)),
            }
        }
    }
}
