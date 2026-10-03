//! The 15 views of §2.12: the frame, then the `s` at every level.

use joinn_visual::Rect;
use joinn_visual::{Camera, ChartId, LEVEL_MAX, LEVEL_MIN, UNIVERSE_SIZE, UniverseLayout, Zoom};

use super::{S_CENTRE, VIEWPORT};

/// `frame`: the universe framed. `at s`: focus on the `s` centre, pinned to
/// `(960, 540)`, at each level from −4 to 9, step 0. Every camera is rebased to
/// the chart under its centre pixel.
pub(crate) fn zoom_views(layout: &UniverseLayout) -> Vec<(String, Camera)> {
    let (w, h) = VIEWPORT;
    let universe = Rect {
        x: 0,
        y: 0,
        w: UNIVERSE_SIZE.0,
        h: UNIVERSE_SIZE.1,
    };
    let mut views = vec![(
        "frame".to_owned(),
        layout.rebase(Camera::frame(universe, ChartId(0), w, h)),
    )];
    for level in LEVEL_MIN..=LEVEL_MAX {
        let camera = Camera {
            zoom: Zoom { level, step: 0 },
            anchor: ChartId(0),
            focus: S_CENTRE,
            pin: (i64::from(w / 2), i64::from(h / 2)),
            width: w,
            height: h,
        };
        views.push(("at s".to_owned(), layout.rebase(camera)));
    }
    views
}
