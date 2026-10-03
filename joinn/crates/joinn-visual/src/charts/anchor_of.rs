//! The anchor a camera should have: the deepest chart under its centre pixel.

use super::UniverseLayout;
use crate::camera::{Camera, ChartId, FOCUS_UNIT};

impl UniverseLayout {
    /// The world point under pixel `(width div 2, height div 2)` is
    /// `focus + (centre − pin)/k` in the camera's anchor chart, a rational with
    /// denominator `2^16·(256 + step)`; it is moved to the root chart and
    /// handed to `chart_at`.
    pub fn anchor_of(&self, camera: &Camera) -> ChartId {
        let base = 256 + i128::from(camera.zoom.step);
        let shift = 24 - camera.zoom.level;
        let den = i128::from(FOCUS_UNIT) * base;
        let origin = self.chart(camera.anchor).map_or((0, 0), |c| c.root_origin);
        let centre = (i64::from(camera.width / 2), i64::from(camera.height / 2));
        let at = |focus: i64, c: i64, pin: i64, o: i64| -> i128 {
            i128::from(focus) * base
                + ((i128::from(c) - i128::from(pin)) << shift)
                + i128::from(o) * den
        };
        self.chart_at(
            (
                at(camera.focus.0, centre.0, camera.pin.0, origin.0),
                at(camera.focus.1, centre.1, camera.pin.1, origin.1),
            ),
            den,
        )
    }
}
