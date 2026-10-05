//! The zoom script of plan 7.2 §2.13 item 1, on the grove.

use joinn_frame::Verdict;
use joinn_visual::{Camera, ChartId, LEVEL_MAX, Rect, UNIVERSE_SIZE};

use super::{S_CENTRE, VIEWPORT};

/// Frame the universe; focus on the `s` with pin `(960, 540)`; one notch at a
/// time about that pin until the level is 9; the same number of notches out.
/// `step` is handed each action's name and camera and returns the camera to
/// continue from (the caller rebases). Returns the notches taken in.
pub(crate) fn zoom_script(
    mut step: impl FnMut(&str, Camera) -> Result<Camera, String>,
) -> Result<usize, String> {
    let (w, h) = VIEWPORT;
    let universe = Rect {
        x: 0,
        y: 0,
        w: UNIVERSE_SIZE.0,
        h: UNIVERSE_SIZE.1,
    };
    let mut camera = step("frame", Camera::frame(universe, ChartId(0), w, h))?;
    let pin = (i64::from(w / 2), i64::from(h / 2));
    let origin = camera.anchor;
    camera = step(
        "focus s",
        Camera {
            anchor: ChartId(0),
            focus: S_CENTRE,
            pin,
            ..camera
        },
    )?;
    if origin != ChartId(0) {
        return Err(format!(
            "zoom script: the frame is anchored at chart {}, not the universe",
            origin.0
        ));
    }
    let mut notches = 0usize;
    while camera.zoom.level < LEVEL_MAX {
        let next = match camera.zoom_about(pin, 1) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        camera = step("in", next)?;
        notches += 1;
    }
    for _ in 0..notches {
        let next = match camera.zoom_about(pin, -1) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        camera = step("out", next)?;
    }
    Ok(notches)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{ChartId, Table, UniverseScene, table_bytes};

    use super::zoom_script;
    use crate::fns::grove::grove_layout;

    #[test]
    fn pans_and_zooms_write_no_row_rebases_write_chart_rows_and_the_end_equals_regrow() {
        let (_, layout) = match grove_layout() {
            Ok(l) => l,
            Err(e) => panic!("{e}"),
        };
        let mut scene = match UniverseScene::grow(layout.clone(), ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let charts = scene.tables().chart.len();
        let mut rebases: Vec<(String, usize)> = Vec::new();
        let notches = zoom_script(|action, camera| {
            if let Some(d) = scene.take_pending() {
                return Err(format!(
                    "{action}: {} row(s) pending before the rebase",
                    d.rows.len()
                ));
            }
            let (camera, delta) = match scene.rebase(camera) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => return Err(r.reason),
            };
            if delta.rows.iter().any(|w| w.table != Table::Chart) {
                return Err(format!(
                    "{action}: a rebase wrote a row that is not a chart row"
                ));
            }
            if delta.rows.len() > charts {
                return Err(format!(
                    "{action}: {} rows for {charts} charts",
                    delta.rows.len()
                ));
            }
            scene.take_pending();
            if !delta.rows.is_empty() {
                rebases.push((action.to_owned(), delta.rows.len()));
            }
            Ok(camera)
        });
        let notches = match notches {
            Ok(n) => n,
            Err(e) => panic!("{e}"),
        };
        println!("zoom script: {notches} notches in and out; rebases {rebases:?}; {charts} charts");
        assert_eq!(notches, 86);
        assert_eq!(rebases.first(), Some(&("focus s".to_owned(), charts)));
        let fresh = match UniverseScene::grow(layout, scene.anchor()) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(table_bytes(scene.tables()), table_bytes(fresh.tables()));
    }
}
