//! Fit a picture's body to a viewport.

use crate::camera::{FitCamera, fit};
use crate::layout::{Layout, Rect};
use crate::tables::Tables;

/// Phase 6's `fit` on body 0's extent; an empty table fits a point.
pub(crate) fn fit_body(tables: &Tables, width: u32, height: u32) -> FitCamera {
    let (w, h) = tables
        .body
        .first()
        .map_or((0, 0), |b| (i64::from(b.w), i64::from(b.h)));
    let extent = Layout {
        surface: Rect { x: 0, y: 0, w, h },
        cells: Vec::new(),
        ports: Vec::new(),
        wires: Vec::new(),
    };
    fit(&extent, width, height)
}
