//! One grove view per band threshold with a body in its fade window.

use std::collections::BTreeSet;

use joinn_visual::{
    Band, Camera, ChartId, ChartKind, CutForm, FOCUS_UNIT, LEVEL_MAX, LEVEL_MIN, STEPS, THRESHOLDS,
    UniverseLayout, Zoom, cut, fade_window, owner_band,
};

use super::VIEWPORT;

/// Per threshold `T`, in order: the first body in chart order (one per
/// distinct size) and the first zoom, ascending, at which the body sits in
/// `T`'s fade window and the higher band owns it, with the camera focused on
/// the body's centre at the viewport's centre and the body drawn in the cut.
/// The label is `fade <T>: <alias> level <L> step <S>`. A threshold with no
/// such view is left out.
pub(crate) fn fade_views(layout: &UniverseLayout) -> Vec<(String, Camera, ChartId)> {
    let (w, h) = VIEWPORT;
    let higher = [Band::Glyph, Band::Summary, Band::Full];
    let mut out = Vec::new();
    for (t, band) in THRESHOLDS.into_iter().zip(higher) {
        let mut sizes = BTreeSet::new();
        let found = (0u32..).zip(&layout.charts).find_map(|(i, c)| {
            let size = c.size.0.max(c.size.1);
            if c.kind != ChartKind::Body || !sizes.insert(size) {
                return None;
            }
            let zoom = (LEVEL_MIN..=LEVEL_MAX)
                .flat_map(|level| (0..STEPS).map(move |step| Zoom { level, step }))
                .find(|&z| fade_window(z, size) == Some(t) && owner_band(z, size) == band)?;
            let camera = layout.rebase(Camera {
                zoom,
                anchor: ChartId(0),
                focus: (
                    (2 * c.root_origin.0 + c.size.0) * FOCUS_UNIT / 2,
                    (2 * c.root_origin.1 + c.size.1) * FOCUS_UNIT / 2,
                ),
                pin: (i64::from(w / 2), i64::from(h / 2)),
                width: w,
                height: h,
            });
            let drawn = cut(layout, &camera)
                .entries
                .iter()
                .any(|e| e.chart == ChartId(i) && matches!(e.form, CutForm::Drawn(_)));
            drawn.then(|| {
                (
                    format!(
                        "fade {t}: {} level {} step {}",
                        c.name, zoom.level, zoom.step
                    ),
                    camera,
                    ChartId(i),
                )
            })
        });
        out.extend(found);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::fade_views;
    use crate::fns::grove::grove_layout;

    #[test]
    fn every_threshold_has_a_drawn_body_in_its_fade_window() {
        let (_, layout) = match grove_layout() {
            Ok(l) => l,
            Err(e) => panic!("{e}"),
        };
        let labels: Vec<String> = fade_views(&layout).into_iter().map(|v| v.0).collect();
        assert_eq!(
            labels,
            [
                "fade 4: b0003 level -3 step 195",
                "fade 32: b0000 level -1 step 195",
                "fade 240: b0000 level 2 step 167",
            ]
        );
    }
}
