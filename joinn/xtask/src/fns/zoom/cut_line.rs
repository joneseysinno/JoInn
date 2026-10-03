//! One view's cut counts, as §2.12 prints them.

use joinn_visual::{Band, Camera, ChartKind, Cut, CutForm, UniverseLayout, print_zoom};

/// `zoom <w>x<h> <label> <zoom>, anchor <chart>: galaxies <n> open, <n> nodes ·
/// systems … · bodies dot …, glyph …, summary …, full … · fading <n>`, where
/// `fading` counts drawn bodies in a crossfade window.
pub(crate) fn cut_line(label: &str, layout: &UniverseLayout, camera: &Camera, cut: &Cut) -> String {
    let mut frames = [[0usize; 2]; 2];
    let mut bands = [0usize; 4];
    let mut fading = 0usize;
    for e in &cut.entries {
        let Some(chart) = layout.chart(e.chart) else {
            continue;
        };
        let row = match chart.kind {
            ChartKind::Galaxy => 0,
            ChartKind::System => 1,
            _ => 2,
        };
        match e.form {
            CutForm::Open if row < 2 => frames[row][0] += 1,
            CutForm::Node if row < 2 => frames[row][1] += 1,
            CutForm::Drawn(band) => {
                let i = match band {
                    Band::Dot => 0,
                    Band::Glyph => 1,
                    Band::Summary => 2,
                    Band::Full => 3,
                };
                bands[i] += 1;
                if e.fading {
                    fading += 1;
                }
            }
            _ => {}
        }
    }
    let anchor = layout
        .chart(camera.anchor)
        .map_or_else(|| "?".to_owned(), |c| c.name.clone());
    format!(
        "zoom {}x{} {label} {}, anchor {anchor}: galaxies {} open, {} nodes · systems {} open, {} nodes · bodies dot {}, glyph {}, summary {}, full {} · fading {fading}",
        camera.width,
        camera.height,
        print_zoom(&camera.zoom),
        frames[0][0],
        frames[0][1],
        frames[1][0],
        frames[1][1],
        bands[0],
        bands[1],
        bands[2],
        bands[3],
    )
}
