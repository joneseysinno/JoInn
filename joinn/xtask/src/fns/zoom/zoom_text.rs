//! Everything `cargo xtask zoom` prints.

use joinn_visual::{LEVEL_MIN, cut, touches};
use std::collections::BTreeSet;
use std::fmt::Write;

use super::{cut_line, zoom_views};
use crate::fns::grove::grove_layout;

/// One cut line per §2.12 view, the touches at level −4 step 0 and at the
/// frame, then `zoom: <n> views, cut counts printed; touches <a> and <b>`. A
/// link that touches one node twice is an error naming it (V146).
pub(crate) fn zoom_text() -> Result<String, String> {
    let (universe, layout) = grove_layout()?;
    let views = zoom_views(&layout);
    let mut out = String::new();
    for (label, camera) in &views {
        let c = cut(&layout, camera);
        let _ = writeln!(out, "{}", cut_line(label, &layout, camera, &c));
    }
    let mut totals = Vec::new();
    let picked = [
        views
            .iter()
            .find(|(l, c)| l == "at s" && c.zoom.level == LEVEL_MIN && c.zoom.step == 0),
        views.iter().find(|(l, _)| l == "frame"),
    ];
    for (label, camera) in picked.into_iter().flatten() {
        let c = cut(&layout, camera);
        let per_link = touches(&layout, &c, &universe.coding.links);
        for (link, nodes) in &per_link {
            let distinct: BTreeSet<_> = nodes.iter().collect();
            if distinct.len() != nodes.len() {
                return Err(format!(
                    "zoom: link {link} touches a node twice; acceptance is each node at most once per link (V146)"
                ));
            }
        }
        let total: usize = per_link.iter().map(|(_, n)| n.len()).sum();
        let name = if label == "frame" {
            "frame".to_owned()
        } else {
            format!("level {} step {}", camera.zoom.level, camera.zoom.step)
        };
        let _ = writeln!(
            out,
            "touches {name}: {} links, {total} touches, each node at most once per link",
            per_link.len()
        );
        totals.push(total.to_string());
    }
    let _ = writeln!(
        out,
        "zoom: {} views, cut counts printed; touches {}",
        views.len(),
        totals.join(" and ")
    );
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::zoom_text;

    #[test]
    fn the_grove_prints_the_plan_s_block_byte_for_byte() {
        let got = match zoom_text() {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        let want = "\
zoom 1920x1080 frame level -2 step 95 (k 351/1024), anchor universe: galaxies 8 open, 0 nodes · systems 128 open, 0 nodes · bodies dot 0, glyph 3072, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -4 step 0 (k 1/16), anchor b0000: galaxies 8 open, 0 nodes · systems 0 open, 128 nodes · bodies dot 0, glyph 0, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -3 step 0 (k 1/8), anchor b0000: galaxies 8 open, 0 nodes · systems 128 open, 0 nodes · bodies dot 1891, glyph 1181, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -2 step 0 (k 1/4), anchor b0000: galaxies 6 open, 0 nodes · systems 96 open, 0 nodes · bodies dot 0, glyph 2240, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -1 step 0 (k 1/2), anchor b0000: galaxies 4 open, 0 nodes · systems 36 open, 0 nodes · bodies dot 0, glyph 864, summary 0, full 0 · fading 0
zoom 1920x1080 at s level 0 step 0 (k 1), anchor b0000: galaxies 1 open, 0 nodes · systems 16 open, 0 nodes · bodies dot 0, glyph 153, summary 113, full 0 · fading 0
zoom 1920x1080 at s level 1 step 0 (k 2), anchor b0000: galaxies 1 open, 0 nodes · systems 4 open, 0 nodes · bodies dot 0, glyph 0, summary 80, full 0 · fading 0
zoom 1920x1080 at s level 2 step 0 (k 4), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 24, full 0 · fading 0
zoom 1920x1080 at s level 3 step 0 (k 8), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 4, full 8 · fading 0
zoom 1920x1080 at s level 4 step 0 (k 16), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 4 · fading 0
zoom 1920x1080 at s level 5 step 0 (k 32), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 2 · fading 0
zoom 1920x1080 at s level 6 step 0 (k 64), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 7 step 0 (k 128), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 8 step 0 (k 256), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 9 step 0 (k 512), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
touches level -4 step 0: 137 links, 264 touches, each node at most once per link
touches frame: 137 links, 1182 touches, each node at most once per link
zoom: 15 views, cut counts printed; touches 264 and 1182
";
        assert_eq!(got, want);
    }
}
