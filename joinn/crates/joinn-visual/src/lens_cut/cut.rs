//! The CPU cut of a laid-out lens under one camera.

use super::{Cut, CutEntry, CutForm};
use crate::bands::{Band, THRESHOLDS, fade_window, owner_band};
use crate::camera::{Camera, ChartId, FOCUS_UNIT};
use crate::charts::{Chart, ChartKind, UniverseLayout};

/// A chart is visible when its pixel rectangle meets the viewport: `x0 < width`,
/// `x0 + k·w > 0`, and the same for y, decided in 2^-32 px. A galaxy, then a
/// system, is open when its owner band is Summary or above, and only an open
/// one's children are walked.
pub fn cut(layout: &UniverseLayout, camera: &Camera) -> Cut {
    let zoom = camera.zoom;
    let anchor = layout
        .chart(camera.anchor)
        .map_or((0, 0), |c| c.root_origin);
    let base = 256 + i128::from(zoom.step);
    let shift = zoom.level + 24;
    let (vw, vh) = (
        i128::from(camera.width) << 32,
        i128::from(camera.height) << 32,
    );
    let visible = |c: &Chart| {
        let (x0, y0) = camera.pixel_of((
            (c.root_origin.0 - anchor.0) * FOCUS_UNIT,
            (c.root_origin.1 - anchor.1) * FOCUS_UNIT,
        ));
        let ext = |n: i64| (base * i128::from(n)) << shift;
        x0 < vw && x0 + ext(c.size.0) > 0 && y0 < vh && y0 + ext(c.size.1) > 0
    };
    let frame = |c: &Chart| {
        let size = c.size.0.max(c.size.1);
        let form = if owner_band(zoom, size) >= Band::Summary {
            CutForm::Open
        } else {
            CutForm::Node
        };
        (form, fade_window(zoom, size) == Some(THRESHOLDS[1]))
    };

    let mut entries = Vec::new();
    let (mut galaxy_open, mut system_open) = (false, false);
    for (i, c) in (0u32..).zip(&layout.charts) {
        let chart = ChartId(i);
        let walked = match c.kind {
            ChartKind::Universe => false,
            ChartKind::Galaxy => {
                system_open = false;
                true
            }
            ChartKind::System => galaxy_open,
            ChartKind::Body => system_open,
        };
        let shown = walked && visible(c);
        let (form, fading) = match c.kind {
            ChartKind::Body => {
                let size = c.size.0.max(c.size.1);
                (
                    CutForm::Drawn(owner_band(zoom, size)),
                    fade_window(zoom, size).is_some(),
                )
            }
            _ => frame(c),
        };
        match c.kind {
            ChartKind::Galaxy => galaxy_open = shown && form == CutForm::Open,
            ChartKind::System => system_open = shown && form == CutForm::Open,
            _ => {}
        }
        if shown {
            entries.push(CutEntry {
                chart,
                form,
                fading,
            });
        }
    }
    Cut { entries }
}
