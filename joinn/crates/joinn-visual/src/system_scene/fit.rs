//! Fit the whole system to a viewport.

use super::SystemScene;
use crate::camera::{FitCamera, fit};
use crate::layout::{Layout, Rect};
use crate::system_layout::SYSTEM_PAD;

impl SystemScene {
    /// Phase 6's `fit` on the system's extent: from the origin to the
    /// response's right side and the body's bottom, each padded by `SYSTEM_PAD`.
    pub fn fit(&self, width: u32, height: u32) -> FitCamera {
        let (s, r) = (self.layout.surface, self.layout.response.rect);
        let extent = Layout {
            surface: Rect {
                x: 0,
                y: 0,
                w: r.x + r.w + SYSTEM_PAD,
                h: (s.y + s.h).max(r.y + r.h) + SYSTEM_PAD,
            },
            cells: Vec::new(),
            ports: Vec::new(),
            wires: Vec::new(),
        };
        fit(&extent, width, height)
    }
}
